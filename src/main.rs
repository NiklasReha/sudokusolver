fn main() {
    let länge_eines_quadrats=3;
    let mut solved =true;
    let mut spielfeld:Vec<Vec<i32>>=vec![vec![3,0,0,4,0,0,0,8,0],
                                        vec![9,5,0,0,0,6,4,3,7],
                                        vec![6,7,0,5,0,0,0,1,0],
                                        vec![0,0,7,0,3,1,0,6,5],
                                        vec![0,0,5,6,0,8,0,0,3],
                                        vec![8,0,0,0,0,0,7,0,1],
                                        vec![0,0,1,3,0,4,0,0,0],
                                        vec![4,0,0,0,0,2,1,5,8],
                                        vec![5,0,0,8,1,9,0,0,4]];
    let mut mögliche_nummern=Vec::new();
    for i in 1..länge_eines_quadrats*länge_eines_quadrats+1{
        mögliche_nummern.push(i as i32);
    }
    
    loop {
        let gebrauchte_nummern:Vec<Vec<i32>>=get_gebrauchte_nummern(&mögliche_nummern, &spielfeld, länge_eines_quadrats);
        let mut changed=false;
        let mut choices =false;
        

        for i in 0..spielfeld.len(){
            for d in 0..spielfeld[i].len(){
                if spielfeld[i][d]==0{
                    let possible_solutions:Vec<i32>=get_possible_numbers(gebrauchte_nummern[get_index(spielfeld[0].len()/länge_eines_quadrats, d/länge_eines_quadrats,i/länge_eines_quadrats)].clone(), &spielfeld, i, d);
                    if possible_solutions.len() == 1 {
                        changed=true;
                        spielfeld[i][d]=possible_solutions[0];
                        break;
                    }
                    else{
                        choices=true;
                    }
                }
            }
            if changed{
                break;
            }
        }
        if !changed{
            for i in 0..spielfeld.len(){
                for d in 0..spielfeld[i].len(){
                    if spielfeld[i][d]==0{
                        solved=false;
                    }
                }
            }
            if solved||choices{
                break;
            }

        }
    }

    println!();
    if solved{
        println!();
        for i in 0..spielfeld.len(){
            for d in 0..spielfeld[i].len(){
                
                if (d+1)%länge_eines_quadrats==0&& d+1<länge_eines_quadrats*länge_eines_quadrats{
                    print!(" {} |",spielfeld[i][d]);
                }
                else{
                    print!(" {} ",spielfeld[i][d]);
                }
            }
            if (i+1)%länge_eines_quadrats==0&& i+1<länge_eines_quadrats*länge_eines_quadrats{
                println!();
                for _i in 0..spielfeld[i].len(){
                    print!("---");
                }
                print!("-");
            }
            println!();
        }
        println!("\nSolved!");
    }
    else{

        for i in 0..spielfeld.len(){
            for d in 0..spielfeld[i].len(){
                
                if (d+1)%länge_eines_quadrats==0&& d+1<länge_eines_quadrats*länge_eines_quadrats{
                    print!(" {} |",spielfeld[i][d]);
                }
                else{
                    print!(" {} ",spielfeld[i][d]);
                }
            }
            if (i+1)%länge_eines_quadrats==0&& i+1<länge_eines_quadrats*länge_eines_quadrats{
                println!();
                for _i in 0..spielfeld[i].len(){
                    print!("---");
                }
                print!("-");
            }
            println!();
        }


        println!("\nCouldnt finish the puzzle!");
    }
}



fn get_gebrauchte_nummern(mögliche_nummern:&Vec<i32>,spielfeld:&Vec<Vec<i32>>,länge_eines_quadrats:usize)->Vec<Vec<i32>>{
    let mut gebrauchte_nummern:Vec<Vec<i32>>=Vec::new();
    for _i in 0..spielfeld[0].len()/länge_eines_quadrats*spielfeld.len()/länge_eines_quadrats{
        gebrauchte_nummern.push(mögliche_nummern.clone());
    }

    for i in 0..spielfeld.len(){
        for d in 0..spielfeld[i].len(){
            let number=spielfeld[i][d];
            if number != 0 &&iscontained(number,&gebrauchte_nummern[get_index(spielfeld[0].len()/länge_eines_quadrats, d/länge_eines_quadrats,i/länge_eines_quadrats)]){
                let index = gebrauchte_nummern[get_index(spielfeld[0].len()/länge_eines_quadrats, d/länge_eines_quadrats,i/länge_eines_quadrats)].iter().position(|x| *x == number).unwrap();
                gebrauchte_nummern[get_index(spielfeld[0].len()/länge_eines_quadrats, d/länge_eines_quadrats,i/länge_eines_quadrats)].remove(index);
            }
        }
    }
    gebrauchte_nummern
}

fn iscontained(number:i32,board:&[i32])->bool{
    let mut iscontained = false;

    for i in board{
        if number==*i{
            iscontained=true;
            break;
        }
    }

    iscontained
}

fn get_possible_numbers(gebrauchte_nummern:Vec<i32>,spielfeld:&Vec<Vec<i32>>,i:usize,d:usize)->Vec<i32>{
    let mut possible_solutions=gebrauchte_nummern.clone();
    for y in 0..spielfeld.len(){
        let number=spielfeld[y][d];
        if number != 0 && iscontained(number, &possible_solutions){
            let index = possible_solutions.iter().position(|x| *x == number).unwrap();
            possible_solutions.remove(index);
        }
    }
    for y in 0..spielfeld[0].len(){
        let number=spielfeld[i][y];
        if number != 0 && iscontained(number, &possible_solutions){
            let index = possible_solutions.iter().position(|x| *x == number).unwrap();
            possible_solutions.remove(index);
        }
    }
    possible_solutions
}

fn get_index(weite:usize,momentane_weite:usize,höhe:usize)->usize{
    höhe*weite+momentane_weite
}
