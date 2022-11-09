use console_engine::pixel;
use console_engine::KeyCode;
use std::f32::consts::PI;
//use std::{thread, time};

// Engine parameters
static WIDTH :u32 = 50;
static HEIGHT:u32 = 50;
static TARGET_FPS:u32 = 5;
static START_X :f64 = WIDTH as f64/2.;
static START_Y :f64 = HEIGHT as f64/2.;
static CENTER_POINT : [f64;2] = [START_X,START_Y];

fn main() {

    // Square parameters
    let side : f64 = 50.;

    //Simulation parameters
    let mut theta: f64 = 0.;
    let theta_step : f64 = PI as f64/16.;
    let mut engine = console_engine::ConsoleEngine::init(WIDTH, HEIGHT, TARGET_FPS).unwrap();

    //Points
    let mut point_a : [f64;2] = [-side/2.,0.];
    let mut point_o : [f64;2] = [side/2.,0.];

    let total_lines : u8 = side as u8/ 4;
    let char_face : char = '.';
    loop {   
        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
        // println!("Boucle");
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen

        // Generate new theta by adding a step
        theta = (theta + theta_step) % (2. * PI as f64) ;

        // generate three vectors based on the rotation factor (0 || PI/2 || 3.PI/4)
        let leading_vector : [f64;2] = rotate_vector(None, theta);
        let leading_vector_t : [f64;2] = rotate_vector(None, theta - PI as f64/2.);
        let leading_vector_tt : [f64;2] = rotate_vector(None, theta + PI as f64 *3./4.);

        // Update the base point of the bottoms corners 
        point_a[0] = CENTER_POINT[0] + leading_vector[0] * (side /2.).sqrt();
        point_a[1] = CENTER_POINT[1] + leading_vector[1] * (side /2.).sqrt();

        point_o[0] = CENTER_POINT[0] + leading_vector_t[0] * (side /2.).sqrt();
        point_o[1] = CENTER_POINT[1] + leading_vector_t[1] * (side /2.).sqrt();

        draw_face(&mut engine, &point_a, &point_o, &leading_vector_tt, total_lines, char_face);
        engine.draw(); // draw the screen
    }
}

// Draw a face by stacking lines of char (char)
// one over the other along a direction (vertex_vector)
// for total_lines times
fn draw_face(engine : &mut console_engine::ConsoleEngine,start_point : &[f64;2], end_point : &[f64;2],vertex_vector : &[f64;2],total_lines : u8, char_face : char){
    let mut a : [f64;2] = [start_point[0],start_point[1]];
    let mut b : [f64;2] = [end_point[0],end_point[1]];

    draw_line_from_points(engine, Some(&a),&b,char_face);  

    for frac_side in 1..total_lines{
        let displacement = frac_side as f64/ total_lines as f64 ;
        a[0] = a[0] + vertex_vector[0] * displacement;
        a[1] = a[1] + vertex_vector[1] * displacement;
        b[0] = b[0] + vertex_vector[0] * displacement;
        b[1] = b[1] + vertex_vector[1] * displacement;
        draw_line_from_points(engine, Some(&a),&b,char_face);   
    }        
    
}

// Draws a line between two points with char (char)
// if the first one isn't supplied, will
// default to CENTER_POINT
fn draw_line_from_points(engine : &mut console_engine::ConsoleEngine,start_point : Option<&[f64;2]>,end_point : &[f64;2],char_face:char){
    match start_point {
        None =>  engine.line(CENTER_POINT[0] as i32, CENTER_POINT[1] as i32, end_point[0] as i32, end_point[1] as i32, pixel::pxl(char_face)),
        Some(start_point) =>   engine.line(start_point[0] as i32, start_point[1] as i32, end_point[0] as i32, end_point[1] as i32, pixel::pxl(char_face))
    }
}


// Rotate a vector by an angle theta using a 2d rotation matrix
// if vector isn't supplied, will default to (Ox) vectorial format
fn rotate_vector(point : Option<&[f64;2]>, theta : f64) -> [f64;2]{
    let mut new_point = [0.,0.];

    match point {
        None=> {
            new_point[0] = - theta.sin();
            new_point[1] =  theta.cos();
        },
        Some(point)=> {
            new_point[0] = theta.cos() * point[0] - theta.sin() * point[1];
            new_point[1] = theta.sin() * point[0] + theta.cos() * point[1];
        }
    }

    new_point[0] = new_point[0] / (new_point[0] * new_point[0] + new_point[1] * new_point[1]).sqrt();
    new_point[1] = new_point[1] / (new_point[0] * new_point[0] + new_point[1] * new_point[1]).sqrt();
    new_point
}

// fn sleep(duration : u64){
//     thread::sleep(time::Duration::from_millis(duration));
// }