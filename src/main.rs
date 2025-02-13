mod snack_game;

fn main() {
    let res=snack_game::test();
    match res {
        Ok(())=>println!("ok"),
        Err(e)=>println!("Err :{}",e)
        
    }
}
