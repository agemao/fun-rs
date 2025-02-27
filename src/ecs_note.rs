//ECS entities,Components,Systems.


//Entity:
struct Entity(u64);
//Component:
#[derive(Components)]
struct Position{
    x:f32,
    y:f32,
}
//System:
fn print_position_system(query: Query<&Position>){
    for position in &query{
        println!("position:{}{}",position.x,position.y);
    }
}

