// --- Uniforms and Storage Buffers ---

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    grid_size: u32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(0) @binding(1) var<storage, read> maze_data: array<u32>;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Hardcoded positions for a full-screen quad (2 triangles)
    const positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), // Triangle 1
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0,  1.0), // Triangle 2
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
    );

    return vec4<f32>(positions[in_vertex_index], 0.0, 1.0);
}

const WALL_TOP: u32 = 1u; 
const WALL_RIGHT: u32 = 2u; 
const WALL_BOTTOM: u32 = 4u; 
const WALL_LEFT: u32 = 8u; 
const CELL_VISITED: u32 = 16u; 
const CELL_BACKTRACK: u32 = 32u; 
const CELL_CURSOR: u32 = 64u; 
const PATH_HORIZONTAL: u32 = 128u; 
const PATH_VERTICAL: u32 = 256u; 
const PATH_UP_LEFT: u32 = 512u;  
const PATH_UP_RIGHT: u32 = 1024u; 
const PATH_DOWN_LEFT: u32 = 2048u; 
const PATH_DOWN_RIGHT: u32 = 4096u; 
const START_LEFT: u32 = 8192u; 
const START_RIGHT: u32 = 16384u; 
const START_UP: u32 = 32768u; 
const START_DOWN: u32 = 65536u; 
const END_LEFT: u32 = 131072u; 
const END_RIGHT: u32 = 262144u; 
const END_UP: u32 = 524288u; 
const END_DOWN: u32 = 1048576u; 
const ARROW_LEFT: u32 = 2097152u; 
const ARROW_RIGHT: u32 = 4194304u; 
const ARROW_UP: u32 = 8388608u; 
const ARROW_DOWN: u32 = 16777216u; 
const CROSSED: u32 = 33554432u; 

const wall_thickness: f32 = 0.1;

    // Define the colors
const wall_color = vec3<f32>(1.0, 1.0, 1.0);
const unvisited_floor_color = vec4<f32>(0.1, 0.1, 0.1, 0.1);
const visited_floor_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
const backtrack_floor_color = vec4<f32>(0.1, 0.0, 0.0, 0.1);
const cursor_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
const cross_color = vec4<f32>(1.0, 0.2, 0.2, 1.0);
const cursor_radius = 0.2;
const MIN_PATH_THICKNESS = 0.45;
const MAX_PATH_THICKNESS = 0.55;
const START_RADIUS = 0.2;
const cross_thickness = 0.1; // Adjust for thicker/thinner lines

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {

    let uv = vec2(
        frag_coord.x / uniforms.resolution.x,
        1.0 - (frag_coord.y / uniforms.resolution.y)
    );

    let screen_aspect = uniforms.resolution.x / uniforms.resolution.y;
    var maze_uv = uv;
    if (screen_aspect > 1.0) {
        maze_uv.x = (uv.x - 0.5) * screen_aspect + 0.5;
    } else {
        maze_uv.y = (uv.y - 0.5) / screen_aspect + 0.5;
    }
    if (maze_uv.x < 0.0 || maze_uv.x > 1.0 || maze_uv.y < 0.0 || maze_uv.y > 1.0) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    let grid_f = f32(uniforms.grid_size);
    let cell_coord = floor(maze_uv * grid_f);
    let cell_index =  u32(cell_coord.x * grid_f + cell_coord.y); 
    if (cell_index >= arrayLength(&maze_data)) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    let cell_data = maze_data[cell_index];

    let is_visited = (cell_data & CELL_VISITED) != 0u;
    let is_backtrack = (cell_data & CELL_BACKTRACK) != 0u;
    let is_cursor = (cell_data & CELL_CURSOR) != 0u;

    var floor_color: vec4<f32>;
    let inner_uv = fract(maze_uv * grid_f); // Pixel's position inside the cell (0.0 to 1.0)
    if (is_cursor) {
        // For the cursor cell, draw a dot in the middle.
        let center = vec2<f32>(0.5, 0.5);
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center < cursor_radius) {
            floor_color = cursor_color; // Inside the dot
        } else {
            floor_color = visited_floor_color; // Outside the dot, on the path
        }    
    } else if ((cell_data & PATH_HORIZONTAL) != 0u) {
        if (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color;
        }    
    } else if ((cell_data & PATH_VERTICAL) != 0u) {
        if (inner_uv.x >= MIN_PATH_THICKNESS && inner_uv.x <= MAX_PATH_THICKNESS) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color;
        }    
    } else if ((cell_data & PATH_UP_LEFT) != 0u) {
        let center = vec2<f32>(0.0, 0.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center >= inner_radius && dist_from_center <= outer_radius) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color;
        }
    } else if ((cell_data & PATH_UP_RIGHT) != 0u) {
        let center = vec2<f32>(1.0, 0.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center >= inner_radius && dist_from_center <= outer_radius) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color;
        }
    } else if ((cell_data & PATH_DOWN_LEFT) != 0u) {
        let center = vec2<f32>(0.0, 1.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center >= inner_radius && dist_from_center <= outer_radius) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color;
        }
    } else if ((cell_data & PATH_DOWN_RIGHT) != 0u) {
        let center = vec2<f32>(1.0, 1.0);
        let inner_radius = MIN_PATH_THICKNESS;
        let outer_radius = MAX_PATH_THICKNESS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center >= inner_radius && dist_from_center <= outer_radius) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color;
        }
    } else if ((cell_data & START_LEFT) != 0u) {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center < radius) {
            floor_color = cursor_color; // Inside the dot
        } else if (inner_uv.x < 0.5 && (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS)) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color; // Outside the dot, on the path
        }    
    } else if ((cell_data & START_RIGHT) != 0u) {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center < radius) {
            floor_color = cursor_color;
        } else if (inner_uv.x > 0.5 && (inner_uv.y >= MIN_PATH_THICKNESS && inner_uv.y <= MAX_PATH_THICKNESS)) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color; // Outside the dot, on the path
        }    
    } else if ((cell_data & START_UP) != 0u) {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center < radius) {
            floor_color = cursor_color; // Inside the dot
        } else if (inner_uv.y < 0.5 && (inner_uv.x >= MIN_PATH_THICKNESS && inner_uv.x <= MAX_PATH_THICKNESS)) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color; // Outside the dot, on the path
        }    
    } else if ((cell_data & START_DOWN) != 0u) {
        let center = vec2<f32>(0.5, 0.5);
        let radius = START_RADIUS;
        let dist_from_center = distance(inner_uv, center);
        if (dist_from_center < radius) {
            floor_color = cursor_color; // Inside the dot
        } else if (inner_uv.y > 0.5 && (inner_uv.x >= MIN_PATH_THICKNESS && inner_uv.x <= MAX_PATH_THICKNESS)) {
            floor_color = cursor_color;
        } else {
            floor_color = visited_floor_color; // Outside the dot, on the path
        }    
    } else if ((cell_data & (END_UP | END_DOWN | END_LEFT | END_RIGHT)) != 0u) {
        let arrow_color = cursor_color;

        var local_uv = inner_uv;
        if ((cell_data & END_LEFT) != 0u) {
            local_uv = vec2<f32>(inner_uv.y, 1.0 - inner_uv.x);
        } else if ((cell_data & END_UP) != 0u) {
            local_uv = vec2<f32>(1.0 - inner_uv.x, 1.0 - inner_uv.y);
        } else if ((cell_data & END_DOWN) != 0u) {
            local_uv = vec2<f32>(1.0 - inner_uv.y, inner_uv.x);
        }

        let path_thickness = MAX_PATH_THICKNESS - MIN_PATH_THICKNESS;
        let arrowhead_half_width = 0.15;

        // Define the arrow's vertical boundaries. It occupies the bottom half of the cell.
        let tip_y = 0.4;            // The tip ends at the center.
        let arrowhead_base_y = 0.7; // The y-position of the arrowhead's flat base.
        let shaft_base_y = 1.0;     // The shaft starts at the cell edge.

        // 1. Check for the shaft (a rectangle from y=1.0 to y=0.7)
        let is_in_shaft = abs(local_uv.x - 0.5) <= path_thickness / 2.0 &&
                          local_uv.y >= arrowhead_base_y && 
                          local_uv.y <= shaft_base_y;

        // 2. Check for the head (a triangle from y=0.7 to y=0.5)
        var is_in_head = false;
        // Bounding box check for efficiency
        if (local_uv.y >= tip_y && local_uv.y < arrowhead_base_y) {
            let center_x = 0.5;
            // The slope of the right edge of the arrowhead
            let slope = (arrowhead_base_y - tip_y) / arrowhead_half_width;

            // Check if the pixel is between the two diagonal lines.
            if (abs(local_uv.x - center_x) * slope <= local_uv.y - tip_y) {
                is_in_head = true;
            }
        }

        if (is_in_shaft || is_in_head) {
            floor_color = arrow_color;
        } else {
            floor_color = visited_floor_color;
        }

    } else if ((cell_data & (ARROW_UP | ARROW_DOWN | ARROW_LEFT | ARROW_RIGHT)) != 0u) {
let arrow_color = cursor_color; // Or any color you want for the arrow

        // We will rotate the inner_uv space so we can use the same "ARROW_UP"
        // drawing logic for all four directions.
        var local_uv = inner_uv;
        if ((cell_data & ARROW_RIGHT) != 0u) {
            // Rotate 90 degrees clockwise
            local_uv = vec2<f32>(inner_uv.y, 1.0 - inner_uv.x);
        } else if ((cell_data & ARROW_DOWN) != 0u) {
            // Rotate 180 degrees
            local_uv = vec2<f32>(1.0 - inner_uv.x, 1.0 - inner_uv.y);
        } else if ((cell_data & ARROW_LEFT) != 0u) {
            // Rotate 90 degrees counter-clockwise
            local_uv = vec2<f32>(1.0 - inner_uv.y, inner_uv.x);
        }

        // --- Arrow Drawing Logic (always for an "up" arrow) ---

        // 1. Define the arrow's geometry and constants
        let path_thickness = 0.03;
        let center_x = 0.5;

        // Overall smaller dimensions for the arrow
        let arrow_height = 0.4;
        let arrow_tip_width = 0.2;
        let arrow_half_width = arrow_tip_width / 2.0;

        // Calculate vertical positions to center the arrow in the cell
        let tip_y = (1.0 - arrow_height) / 2.0;  // Top-most point of the arrow (y=0.3)
        let base_y = tip_y + arrow_height;       // Bottom-most point of the shaft (y=0.7)

        // Define the arrowhead's shape. Let's make its height proportional to its width.
        let arrowhead_height = arrow_tip_width;
        // The y-coordinate where the head's diagonal lines end
        let arrowhead_base_y = tip_y + arrowhead_height;

        // 2. Check for the shaft (a vertical line)
        // The shaft now goes all the way from the base to the tip.
        let is_in_shaft = abs(local_uv.x - center_x) < path_thickness / 2.0 &&
                          local_uv.y >= tip_y && local_uv.y <= base_y;

        // 3. Check for the head (two lines forming a chevron ^)
        var is_in_head = false;
        // Bounding box for the head, which starts at the tip and extends down
        if (local_uv.y >= tip_y && local_uv.y <= arrowhead_base_y) {
            // Slope of the arrowhead's right edge
            let slope = arrowhead_height / arrow_half_width;

            // For any given x, calculate the corresponding y on the V-shaped line
            let y_on_line = tip_y + slope * abs(local_uv.x - center_x);

            // Check if the fragment's y is close to the line's y
            if (abs(local_uv.y - y_on_line) < path_thickness / 2.0) {
                is_in_head = true;
            }
        }

        if (is_in_shaft || is_in_head) {
            floor_color = arrow_color;
        } else if (is_backtrack) {
            floor_color = backtrack_floor_color;
        } else {
            floor_color = visited_floor_color;
        }

//         let arrow_color = cursor_color; // Or any color you want for the arrow
// 
//         // We will rotate the inner_uv space so we can use the same "ARROW_UP"
//         // drawing logic for all four directions.
//         var local_uv = inner_uv;
//         if ((cell_data & ARROW_RIGHT) != 0u) {
//             // Rotate 90 degrees clockwise
//             local_uv = vec2<f32>(inner_uv.y, 1.0 - inner_uv.x);
//         } else if ((cell_data & ARROW_DOWN) != 0u) {
//             // Rotate 180 degrees
//             local_uv = vec2<f32>(1.0 - inner_uv.x, 1.0 - inner_uv.y);
//         } else if ((cell_data & ARROW_LEFT) != 0u) {
//             // Rotate 90 degrees counter-clockwise
//             local_uv = vec2<f32>(1.0 - inner_uv.y, inner_uv.x);
//         }
// 
//         // --- Arrow Drawing Logic (always for an "up" arrow) ---
//         let path_thickness = 0.01;
//         let arrowhead_base_y = 0.4; // The y-position of the arrowhead's flat base
//         let arrowhead_half_width = 0.3; // Arrowhead goes from 0.5-0.3 to 0.5+0.3
// 
//         // 1. Check for the shaft (a simple rectangle)
//         let is_in_shaft = abs(local_uv.x - 0.5) <= path_thickness / 2.0 &&
//                           local_uv.y >= arrowhead_base_y;
// 
//         // 2. Check for the head (a triangle)
//         // Bounding box check first for efficiency
//         var is_in_head = false;
//         if (local_uv.y < arrowhead_base_y) {
//             // The two diagonal lines of the arrowhead
//             let center_x = 0.5;
//             let tip_y = 0.0;
//             // The slope of the right edge of the arrowhead
//             let slope = (arrowhead_base_y - tip_y) / arrowhead_half_width;
// 
//             // Check if the pixel is between the two diagonal lines
//             if (abs(local_uv.x - center_x) * slope <= local_uv.y - tip_y) {
//                 is_in_head = true;
//             }
//         }
// 
//         if (is_in_shaft || is_in_head) {
//             floor_color = arrow_color;
//         } else if (is_backtrack) {
//             floor_color = backtrack_floor_color;
//         } else {
//             floor_color = visited_floor_color;
//         }

    } else if ((cell_data & CROSSED) != 0u) {
        let on_diag1 = abs(inner_uv.x - inner_uv.y) < cross_thickness / sqrt(2.0);
        let on_diag2 = abs(inner_uv.x + inner_uv.y - 1.0) < cross_thickness / sqrt(2.0);
        if (on_diag1 || on_diag2) {
            floor_color = cross_color;
        } else {
            floor_color = visited_floor_color;
        }
    } else if (is_backtrack) {
      floor_color = backtrack_floor_color;
    } else if (is_visited) {
      floor_color = visited_floor_color;
    }

    // draw the walls
     let has_top_wall    = (cell_data & WALL_TOP) != 0u;
     let has_bottom_wall = (cell_data & WALL_BOTTOM) != 0u;
     let has_left_wall   = (cell_data & WALL_LEFT) != 0u;
     let has_right_wall  = (cell_data & WALL_RIGHT) != 0u;
 
     var min_dist = 1.0;
     if (has_top_wall)    { min_dist = min(min_dist, inner_uv.y); }
     if (has_bottom_wall) { min_dist = min(min_dist, 1.0 - inner_uv.y); }
     if (has_left_wall)   { min_dist = min(min_dist, inner_uv.x); }
     if (has_right_wall)  { min_dist = min(min_dist, 1.0 - inner_uv.x); }
 
     if (min_dist < wall_thickness) {
         let wall_alpha = 1.0 - smoothstep(0.0, wall_thickness, min_dist);
         return vec4(wall_color * wall_alpha, wall_alpha);
     }

    return floor_color;
}
