const SHAPE_TYPE_SPHERE: u32 = 0;
const SHAPE_TYPE_TRIANGLE: u32 = 1;
const SHAPE_TYPE_BOX: u32 = 2;
const SHAPE_TYPE_LINE: u32 = 3;
const SHAPE_TYPE_CIRCLE_SECTOR: u32 = 4;
const SHAPE_TYPE_POLYQUAD: u32 = 5;
const SHAPE_TYPE_SENTINEL: u32 = 0xFFFFFFFF;

struct Shape {
    shape_type: u32,
    distance_offset: f32,
    _padding: vec2<f32>,
    color: vec4<f32>,

    params: array<f32, 8>,
}

@group(0) @binding(0)
var<storage, read> shapes: array<Shape>;

@vertex
fn main_vs(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Generate a fullscreen triangle
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );
    return vec4<f32>(pos[vertex_index], 0.0, 1.0);
}

@fragment
fn main_fs(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    // Go through all shapes
    for(var i: u32 = 0u; i < arrayLength(&shapes); i = i + 1u) {
        let shape = shapes[i];
        if(shape.shape_type == SHAPE_TYPE_SENTINEL) {
            break;
        }

        let frag_pos: vec2<f32> = frag_coord.xy;
        var dist = sd_shape(frag_pos, shape);

        dist += shape.distance_offset;
        color = mix(color, shape.color, clamp(1 - dist, 0.0, 1.0) * shape.color.a);
    }

    return color;
}

fn sd_shape(p: vec2<f32>, shape: Shape) -> f32 {
    switch(shape.shape_type) {
        case SHAPE_TYPE_SPHERE: {
            let pos = vec2<f32>(shape.params[0], shape.params[1]);
            let radius = shape.params[2];
            return sd_circle(p - pos, radius);
        }
        case SHAPE_TYPE_TRIANGLE: {
            let p0 = vec2<f32>(shape.params[0], shape.params[1]);
            let p1 = vec2<f32>(shape.params[2], shape.params[3]);
            let p2 = vec2<f32>(shape.params[4], shape.params[5]);
            return sd_triangle(p, p0, p1, p2);
        }
        case SHAPE_TYPE_BOX: {
            let pos = vec2<f32>(shape.params[0], shape.params[1]);
            let extents = vec2<f32>(shape.params[2], shape.params[3]);
            let corner_radii = vec4<f32>(
                shape.params[4],
                shape.params[5],
                shape.params[6],
                shape.params[7]
            );
            return sd_rounded_box(p - pos, extents, corner_radii);
        }
        case SHAPE_TYPE_LINE: {
            let a = vec2<f32>(shape.params[0], shape.params[1]);
            let b = vec2<f32>(shape.params[2], shape.params[3]);
            return sd_line(p, a, b);
        }
        case SHAPE_TYPE_CIRCLE_SECTOR: {
            let pos = vec2<f32>(shape.params[0], shape.params[1]);
            let radius_inner = shape.params[2];
            let radius_outer = shape.params[3];
            let angle_start = shape.params[4];
            let angle_end = shape.params[5];
            return sd_sector(p - pos, radius_inner, radius_outer, angle_start, angle_end);
        }
        case SHAPE_TYPE_POLYQUAD: {
            let v0 = vec2<f32>(shape.params[0], shape.params[1]);
            let v1 = vec2<f32>(shape.params[2], shape.params[3]);
            let v2 = vec2<f32>(shape.params[4], shape.params[5]);
            let v3 = vec2<f32>(shape.params[6], shape.params[7]);
            let vertices = array<vec2<f32>, 4>(v0, v1, v2, v3);
            return sd_quad(p, vertices);
        }
        default: {
            return 1e6; // Large distance for unsupported shapes
        }
    }
}

// SDF functions (https://iquilezles.org/articles/distfunctions2d/)
fn sd_circle(p: vec2<f32>, radius: f32) -> f32 {
    return length(p) - radius;
}

// fn sd_box(p: vec2<f32>, b: vec2<f32>) -> f32 {
//     let d = abs(p) - b;
//     return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
// }

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    var radii = vec2<f32>(0.0, 0.0);
    if p.x > 0.0 {
        radii.x = r.x;
        radii.y = r.y;
    } else {
        radii.x = r.z;
        radii.y = r.w;
    }
    if p.y > 0.0 {
        radii.x = radii.x;
    } else {
        radii.x = radii.y;
    }

    let q = abs(p)-b+radii.x;
    return min(max(q.x,q.y),0.0) + length(max(q, vec2(0.0, 0.0))) - radii.x;
}

fn sd_line(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

fn sd_triangle(p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>) -> f32
{
    let e0 = p1-p0; let e1 = p2-p1; let e2 = p0-p2;
    let v0 = p -p0; let v1 = p -p1; let v2 = p -p2;
    let pq0 = v0 - e0*clamp( dot(v0,e0)/dot(e0,e0), 0.0, 1.0 );
    let pq1 = v1 - e1*clamp( dot(v1,e1)/dot(e1,e1), 0.0, 1.0 );
    let pq2 = v2 - e2*clamp( dot(v2,e2)/dot(e2,e2), 0.0, 1.0 );
    let s = sign( e0.x*e2.y - e0.y*e2.x );
    let d = min(min(vec2(dot(pq0,pq0), s*(v0.x*e0.y-v0.y*e0.x)),
                     vec2(dot(pq1,pq1), s*(v1.x*e1.y-v1.y*e1.x))),
                     vec2(dot(pq2,pq2), s*(v2.x*e2.y-v2.y*e2.x)));
    return -sqrt(d.x)*sign(d.y);
}

fn sd_sector(p: vec2<f32>, ir: f32, or: f32, a1: f32, a2: f32) -> f32 {
    let TAU: f32 = 6.283185307179586;
    var delta: f32 = a2 - a1;
    delta = delta - floor(delta / TAU) * TAU;

    if (delta <= 1e-6) {
        return length(p);
    }
    if (delta >= TAU - 1e-6) {
        let r: f32 = length(p);
        return max(r - or, ir - r);
    }

    let mid: f32 = a1 + 0.5 * delta;
    let cm: f32 = cos(mid);
    let sm: f32 = sin(mid);
    let q: vec2<f32> = vec2<f32>(cm * p.x + sm * p.y, -sm * p.x + cm * p.y);

    let rlen: f32 = length(q);
    let theta: f32 = atan2(q.y, q.x);
    let half: f32 = 0.5 * delta;

    if (abs(theta) <= half) {
        if (rlen < ir) {
            return ir - rlen;
        } else if (rlen > or) {
            return rlen - or;
        } else {
            let d_in: f32 = rlen - ir;
            let d_out: f32 = or - rlen;
            return -min(d_in, d_out);
        }
    }

    var sign: f32 = -1.0;
    if (theta > 0.0) { sign = 1.0; }

    let ch: f32 = cos(half);
    let sh: f32 = sin(half);
    let u: vec2<f32> = vec2<f32>(ch, sign * sh);

    let t: f32 = dot(q, u);
    let tclamped: f32 = clamp(t, ir, or);

    let closest: vec2<f32> = u * tclamped;
    return length(q - closest); // positive (outside)
}

fn sd_quad(p: vec2<f32>, v: array<vec2<f32>, 4>) -> f32 {
    let N: u32 = 4u;

    var d: f32 = dot(p - v[0], p - v[0]);
    var s: f32 = 1.0;

    var j: u32 = N - 1u;

    for (var i: u32 = 0u; i < N; i = i + 1u) {
        let vi = v[i];
        let vj = v[j];

        let e = vj - vi;
        let w = p - vi;

        let t = clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
        let b = w - e * t;

        d = min(d, dot(b, b));

        let cross = e.x * w.y - e.y * w.x;

        if ((p.y >= vi.y && p.y < vj.y && cross > 0.0) ||
            (p.y <  vi.y && p.y >= vj.y && cross < 0.0)) {
            s = -s;
        }

        j = i;
    }

    return s * sqrt(d);
}
