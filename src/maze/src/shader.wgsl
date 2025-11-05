struct Colors {
    wall_color: vec4<f32>,
    unvisited_floor_color: vec4<f32>,
    visited_floor_color: vec4<f32>,
    backtrack_floor_color: vec4<f32>,
    cursor_color: vec4<f32>,
    cross_color: vec4<f32>,
}

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    grid_size: u32,
    colors: Colors,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(0) @binding(1) var<storage, read> maze_data: array<vec2<u32>>;

var<private> POSITIONS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0), // Triangle 1
    vec2<f32>(1.0, -1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0, 1.0), // Triangle 2
    vec2<f32>(1.0, -1.0),
    vec2<f32>(1.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(POSITIONS[in_vertex_index], 0.0, 1.0);
}

const WALL_TOP:    u32 = 1u << 0u;
const WALL_RIGHT:  u32 = 1u << 1u;
const WALL_BOTTOM: u32 = 1u << 2u;
const WALL_LEFT:   u32 = 1u << 3u;
const CELL_VISITED:   u32 = 1u << 4u;
const CELL_BACKTRACK: u32 = 1u << 5u;
const CELL_CURSOR:    u32 = 1u << 6u;
const PATH_HORIZONTAL: u32 = 1u << 7u;
const PATH_VERTICAL:   u32 = 1u << 8u;
const PATH_UP_LEFT:    u32 = 1u << 9u;
const PATH_UP_RIGHT:   u32 = 1u << 10u;
const PATH_DOWN_LEFT:  u32 = 1u << 11u;
const PATH_DOWN_RIGHT: u32 = 1u << 12u;
const START_LEFT:  u32 = 1u << 13u;
const START_RIGHT: u32 = 1u << 14u;
const START_UP:    u32 = 1u << 15u;
const START_DOWN:  u32 = 1u << 16u;
const END_LEFT:  u32 = 1u << 17u;
const END_RIGHT: u32 = 1u << 18u;
const END_UP:    u32 = 1u << 19u;
const END_DOWN:  u32 = 1u << 20u;
const ARROW_LEFT:  u32 = 1u << 21u;
const ARROW_RIGHT: u32 = 1u << 22u;
const ARROW_UP:    u32 = 1u << 23u;
const ARROW_DOWN:  u32 = 1u << 24u;
const CROSSED: u32 = 1u << 25u;
const CELL_WEIGHT: u32 = 1u << 26u;
const USE_WALL_FOLLOWER_PATH: u32 = 1u << 27u;

const WF_TURN_TOP_RIGHT: u32 = 1u << 4;
const WF_TURN_TOP_LEFT: u32 = 1u << 5;
const WF_TURN_BOTTOM_RIGHT: u32 = 1u << 6;
const WF_TURN_BOTTOM_LEFT: u32 = 1u << 7;

const wall_thickness: f32 = 0.1;
  
const cursor_radius = 0.2;
const MIN_PATH_THICKNESS = 0.45;
const MAX_PATH_THICKNESS = 0.55;
const START_RADIUS = 0.2;
const cross_thickness = 0.1;

fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - abs(h_prime % 2.0 - 1.0));
    let m = v - c;

    var rgb_prime: vec3<f32>;
    if h_prime < 1.0 {
        rgb_prime = vec3<f32>(c, x, 0.0);
    } else if h_prime < 2.0 {
        rgb_prime = vec3<f32>(x, c, 0.0);
    } else if h_prime < 3.0 {
        rgb_prime = vec3<f32>(0.0, c, x);
    } else if h_prime < 4.0 {
        rgb_prime = vec3<f32>(0.0, x, c);
    } else if h_prime < 5.0 {
        rgb_prime = vec3<f32>(x, 0.0, c);
    } else {
        rgb_prime = vec3<f32>(c, 0.0, x);
    }

    return rgb_prime + vec3<f32>(m, m, m);
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {

    let uv = vec2(
        frag_coord.x / uniforms.resolution.x,
        /*1.0 -*/ (frag_coord.y / uniforms.resolution.y)
    );

    let screen_aspect = uniforms.resolution.x / uniforms.resolution.y;
    var maze_uv = uv;
    if screen_aspect > 1.0 {
        maze_uv.x = (uv.x - 0.5) * screen_aspect + 0.5;
    } else {
        maze_uv.y = (uv.y - 0.5) / screen_aspect + 0.5;
    }
    if maze_uv.x < 0.0 || maze_uv.x > 1.0 || maze_uv.y < 0.0 || maze_uv.y > 1.0 {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    let grid_f = f32(uniforms.grid_size);
    let cell_coord = floor(maze_uv * grid_f);
    let cell_index = u32(cell_coord.y * grid_f + cell_coord.x); 
    if cell_index >= arrayLength(&maze_data) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    let cell_data = maze_data[cell_index];

    let is_visited = (cell_data.x & CELL_VISITED) != 0u;
    let is_backtrack = (cell_data.x & CELL_BACKTRACK) != 0u;
    let is_cursor = (cell_data.x & CELL_CURSOR) != 0u;

    var floor_color: vec4<f32> = uniforms.colors.unvisited_floor_color;
    let inner_uv = fract(maze_uv * grid_f); // Pixel's position inside the cell (0.0 to 1.0)
    if is_cursor {
        let center = vec2<f32>(0.5, 0.5);
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center < cursor_radius {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & PATH_HORIZONTAL) != 0u {
        if inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & PATH_VERTICAL) != 0u {
        if inner_uv.x >= MIN_PATH_THICKNESS && inner_uv.x <= MAX_PATH_THICKNESS {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & PATH_UP_LEFT) != 0u {
        let center = vec2<f32>(0.0, 0.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center >= inner_radius && dist_from_center <= outer_radius {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & PATH_UP_RIGHT) != 0u {
        let center = vec2<f32>(1.0, 0.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center >= inner_radius && dist_from_center <= outer_radius {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & PATH_DOWN_LEFT) != 0u {
        let center = vec2<f32>(0.0, 1.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center >= inner_radius && dist_from_center <= outer_radius {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & PATH_DOWN_RIGHT) != 0u {
        let center = vec2<f32>(1.0, 1.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center >= inner_radius && dist_from_center <= outer_radius {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & START_LEFT) != 0u {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center < radius {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.x < 0.5 && (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS) {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & START_RIGHT) != 0u {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center < radius {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.x > 0.5 && (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS) {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & START_UP) != 0u {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center < radius {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.y < 0.5 && (inner_uv.x >= MIN_PATH_THICKNESS && inner_uv.x <= MAX_PATH_THICKNESS) {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & START_DOWN) != 0u {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center < radius {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.y > 0.5 && (inner_uv.x >= MIN_PATH_THICKNESS && inner_uv.x <= MAX_PATH_THICKNESS) {
            floor_color =  uniforms.colors.cursor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & (END_UP | END_DOWN | END_LEFT | END_RIGHT)) != 0u {
        let arrow_color =  uniforms.colors.cursor_color;

        var local_uv = inner_uv;
        if (cell_data.x & END_RIGHT) != 0u {
            local_uv = vec2<f32>(inner_uv.y, 1.0 - inner_uv.x);
        } else if (cell_data.x & END_DOWN) != 0u {
            local_uv = vec2<f32>(1.0 - inner_uv.x, 1.0 - inner_uv.y);
        } else if (cell_data.x & END_LEFT) != 0u {
            local_uv = vec2<f32>(1.0 - inner_uv.y, inner_uv.x);
        }

        let path_thickness = MAX_PATH_THICKNESS - MIN_PATH_THICKNESS;
        let arrowhead_half_width = 0.15;

        let tip_y = 0.4;
        let arrowhead_base_y = 0.7;
        let shaft_base_y = 1.0;

        let is_in_shaft = abs(local_uv.x - 0.5) <= path_thickness / 2.0 && local_uv.y >= arrowhead_base_y && local_uv.y <= shaft_base_y;

        var is_in_head = false;
        if local_uv.y >= tip_y && local_uv.y < arrowhead_base_y {
            let center_x = 0.5;
            let slope = (arrowhead_base_y - tip_y) / arrowhead_half_width;
            if abs(local_uv.x - center_x) * slope <= local_uv.y - tip_y {
                is_in_head = true;
            }
        }

        if is_in_shaft || is_in_head {
            floor_color = arrow_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & (ARROW_UP | ARROW_DOWN | ARROW_LEFT | ARROW_RIGHT)) != 0u {
        let arrow_color =  uniforms.colors.cursor_color;

        var local_uv = inner_uv;
        if (cell_data.x & ARROW_RIGHT) != 0u {
            local_uv = vec2<f32>(inner_uv.y, 1.0 - inner_uv.x);
        } else if (cell_data.x & ARROW_DOWN) != 0u {
            local_uv = vec2<f32>(1.0 - inner_uv.x, 1.0 - inner_uv.y);
        } else if (cell_data.x & ARROW_LEFT) != 0u {
            local_uv = vec2<f32>(1.0 - inner_uv.y, inner_uv.x);
        }

        let path_thickness = 0.03;
        let center_x = 0.5;

        let arrow_height = 0.4;
        let arrow_tip_width = 0.2;
        let arrow_half_width = arrow_tip_width / 2.0;

        let tip_y = (1.0 - arrow_height) / 2.0;
        let base_y = tip_y + arrow_height;

        let arrowhead_height = arrow_tip_width;
        let arrowhead_base_y = tip_y + arrowhead_height;

        let is_in_shaft = abs(local_uv.x - center_x) < path_thickness / 2.0 && local_uv.y >= tip_y && local_uv.y <= base_y;

        var is_in_head = false;
        if local_uv.y >= tip_y && local_uv.y <= arrowhead_base_y {
            let slope = arrowhead_height / arrow_half_width;
            let y_on_line = tip_y + slope * abs(local_uv.x - center_x);
            if abs(local_uv.y - y_on_line) < path_thickness / 2.0 {
                is_in_head = true;
            }
        }

        if is_in_shaft || is_in_head {
            floor_color = arrow_color;
        } else if is_backtrack {
            floor_color =  uniforms.colors.backtrack_floor_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & CROSSED) != 0u {
        let on_diag1 = abs(inner_uv.x - inner_uv.y) < cross_thickness / sqrt(2.0);
        let on_diag2 = abs(inner_uv.x + inner_uv.y - 1.0) < cross_thickness / sqrt(2.0);
        if on_diag1 || on_diag2 {
            floor_color =  uniforms.colors.cross_color;
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }
    } else if (cell_data.x & CELL_WEIGHT) != 0u {
        let center = vec2<f32>(0.5, 0.5);
        let dist_from_center = distance(inner_uv, center);
        if dist_from_center < cursor_radius {
            let t = f32(cell_data.y) / max(1.0, f32(uniforms.grid_size * uniforms.grid_size));
            let hue = 240.0 - (t * 240.0);
            let saturation = 1.0;
            let value = 1.0;

            let path_color_rgb = hsv2rgb(vec3<f32>(hue, saturation, value));

            floor_color = vec4<f32>(path_color_rgb, 1.0);
        } else {
            floor_color =  uniforms.colors.visited_floor_color;
        }



    } else if is_backtrack {
        floor_color =  uniforms.colors.backtrack_floor_color;
    } else if is_visited {
        floor_color =  uniforms.colors.visited_floor_color;
    }


     if (cell_data.x & USE_WALL_FOLLOWER_PATH) != 0u {
      if (cell_data.y & WALL_TOP) != 0 && (cell_data.y & WALL_RIGHT) != 0u {
        if inner_uv.y >= 0.1 && inner_uv.y <= 0.2 {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.x >= 0.8 && inner_uv.x <= 0.9 {
            floor_color =  uniforms.colors.cursor_color;
        }
      } else if (cell_data.y & WALL_BOTTOM) != 0 && (cell_data.y & WALL_RIGHT) != 0u {
        if inner_uv.y >= 0.8 && inner_uv.y <= 0.9 {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.x >= 0.8 && inner_uv.x <= 0.9 {
            floor_color =  uniforms.colors.cursor_color;
        }
      } else if (cell_data.y & WALL_TOP) != 0 && (cell_data.y & WALL_LEFT) != 0u {
        if inner_uv.y >= 0.1 && inner_uv.y <= 0.2 {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.x >= 0.1 && inner_uv.x <= 0.2 {
            floor_color =  uniforms.colors.cursor_color;
        }
      } else if (cell_data.y & WALL_BOTTOM) != 0 && (cell_data.y & WALL_LEFT) != 0u {
        if inner_uv.y >= 0.8 && inner_uv.y <= 0.9 {
            floor_color =  uniforms.colors.cursor_color;
        } else if inner_uv.x >= 0.1 && inner_uv.x <= 0.2 {
            floor_color =  uniforms.colors.cursor_color;
        }
      } else if (cell_data.y & WALL_TOP) != 0u {
        if inner_uv.y >= 0.1 && inner_uv.y <= 0.2 {
            floor_color =  uniforms.colors.cursor_color;
//        } else {
//            floor_color =  uniforms.colors.visited_floor_color;
        }
      } else if (cell_data.y & WALL_BOTTOM) != 0u {
        if inner_uv.y >= 0.8 && inner_uv.y <= 0.9 {
            floor_color =  uniforms.colors.cursor_color;
//        } else {
//            floor_color =  uniforms.colors.visited_floor_color;
        }
      } else if (cell_data.y & WALL_LEFT) != 0u {
        if inner_uv.x >= 0.1 && inner_uv.x <= 0.2 {
            floor_color =  uniforms.colors.cursor_color;
//        } else {
//            floor_color =  uniforms.colors.visited_floor_color;
        }
      } else if (cell_data.y & WALL_RIGHT) != 0u {
        if inner_uv.x >= 0.8 && inner_uv.x <= 0.9 {
            floor_color =  uniforms.colors.cursor_color;
//        } else {
 //           floor_color =  uniforms.colors.visited_floor_color;
        }

      } else if (cell_data.y & WF_TURN_BOTTOM_LEFT) != 0u { 
        let center = vec2<f32>(1.0, 0.0); 
        let inner_radius = 0.1; 
        let outer_radius = 0.2; 
        let dist = distance(inner_uv, center); 
        if dist >= inner_radius && dist <= outer_radius { 
            floor_color = uniforms.colors.cursor_color; 
        } 
      } else if (cell_data.y & WF_TURN_BOTTOM_RIGHT) != 0u { 
        let center = vec2<f32>(0.0, 0.0); 
        let inner_radius = 0.1; 
        let outer_radius = 0.2; 
        let dist = distance(inner_uv, center); 
        if dist >= inner_radius && dist <= outer_radius { 
            floor_color = uniforms.colors.cursor_color; 
        } 
      } else if (cell_data.y & WF_TURN_TOP_LEFT) != 0u { 
        let center = vec2<f32>(1.0, 1.0); 
        let inner_radius = 0.1; 
        let outer_radius = 0.2; 
        let dist = distance(inner_uv, center); 
        if dist >= inner_radius && dist <= outer_radius { 
            floor_color = uniforms.colors.cursor_color; 
        } 
      } else if (cell_data.y & WF_TURN_TOP_RIGHT) != 0u { 
        let center = vec2<f32>(0.0, 1.0); 
        let inner_radius = 0.1; 
        let outer_radius = 0.2; 
        let dist = distance(inner_uv, center); 
        if dist >= inner_radius && dist <= outer_radius { 
            floor_color = uniforms.colors.cursor_color; 
        } 
      } 


















//      } else if (cell_data.y & WF_TURN_TOP_RIGHT) != 0u {
//
//        let center = vec2<f32>(0.75, 0.75);
//        let radius = START_RADIUS;
//        let dist_from_center = distance(inner_uv, center);
//        if dist_from_center < 0.2 {
//            floor_color =  uniforms.colors.cursor_color;
//        } else if inner_uv.x > 0.1 && (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS) {
//            floor_color =  uniforms.colors.cursor_color;
////        } else {
////            floor_color =  uniforms.colors.visited_floor_color;
//        }
//      } else if (cell_data.y & WF_TURN_TOP_LEFT) != 0u {
//
//        let center = vec2<f32>(0.75, 0.75);
//        let radius = START_RADIUS;
//        let dist_from_center = distance(inner_uv, center);
//        if dist_from_center < 0.2 {
//            floor_color =  uniforms.colors.cursor_color;
//        } else if inner_uv.x > 0.1 && (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS) {
//            floor_color =  uniforms.colors.cursor_color;
// //       } else {
//  //          floor_color =  uniforms.colors.visited_floor_color;
//        }
//     }
     }


    // draw the walls
    let has_top_wall = (cell_data.x & WALL_TOP) != 0u;
    let has_bottom_wall = (cell_data.x & WALL_BOTTOM) != 0u;
    let has_left_wall = (cell_data.x & WALL_LEFT) != 0u;
    let has_right_wall = (cell_data.x & WALL_RIGHT) != 0u;

    var min_dist = 1.0;
    if has_top_wall { min_dist = min(min_dist, inner_uv.y); }
    if has_bottom_wall { min_dist = min(min_dist, 1.0 - inner_uv.y); }
    if has_left_wall { min_dist = min(min_dist, inner_uv.x); }
    if has_right_wall { min_dist = min(min_dist, 1.0 - inner_uv.x); }

    if min_dist < wall_thickness {
        let wall_alpha = 1.0 - smoothstep(0.0, wall_thickness, min_dist);
        return vec4(uniforms.colors.wall_color.rgb, wall_alpha);
    }

    return floor_color;
}
