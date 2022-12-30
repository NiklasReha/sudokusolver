use crossterm::{ExecutableCommand,cursor::{MoveToPreviousLine,Hide,
    Show}};
use std::io::{stdout};
fn main() {
    let länge_eines_quadrats=3;
    let mut solved =true;
    let mut spielfeld:Vec<Vec<i32>>=vec![vec![0,0,7,0,5,6,0,0,0],
                                        vec![0,0,1,9,0,0,0,0,0],
                                        vec![4,0,0,1,2,0,0,0,0],
                                        vec![6,0,0,0,0,0,8,7,0],
                                        vec![7,0,4,0,0,0,6,0,1],
                                        vec![0,2,8,0,0,0,0,0,9],
                                        vec![0,0,0,0,9,1,0,0,8],
                                        vec![0,0,0,0,0,5,4,0,0],
                                        vec![0,0,0,6,3,0,2,0,0]];
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
        print_spielfeld(&spielfeld, länge_eines_quadrats);
        println!("\nSolved!");
    }
    else{
        print_spielfeld(&spielfeld, länge_eines_quadrats);
        println!("\nCouldnt finish the puzzle!\nNow trying to guess...\n");
        println!("Schritte des Algorithmus:\n");
        if !solve_sudoku(&mut spielfeld, länge_eines_quadrats){
            println!("Couldnt finish it :(\n");
        }
        else{
            println!();
            println!();
            print_spielfeld(&spielfeld, länge_eines_quadrats);
            println!("\nThe final result!");
        }

    }
}

fn print_spielfeld(spielfeld:&Vec<Vec<i32>>,länge_eines_quadrats:usize){
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

fn solve_sudoku(spielfeld:&mut Vec<Vec<i32>>,länge_eines_quadrats:usize)->bool{
    for col in 0..spielfeld.len(){
        progressbar(spielfeld.clone(), col, 20);
        for row in 0..spielfeld[0].len(){
            if spielfeld[col][row]==0{
                for number in 0..länge_eines_quadrats*länge_eines_quadrats+1{
                    if is_allowed(spielfeld, row, col, number as i32, länge_eines_quadrats){
                        spielfeld[col][row]=number as i32;
                        if solve_sudoku(spielfeld, länge_eines_quadrats){
                            return true;
                        }
                        else{
                            spielfeld[col][row]=0;
                        }
                    }
                }
                return false;
            }
        }
    }
    true
}

fn progressbar(list: impl IntoIterator,index:usize,steps:usize){
    let mut size:usize= list.into_iter().count();
    if size>=1{
        size-=1;
    }
    let mut repeat=((index as f32/size as f32)*(steps+1)as f32) as usize;
    if repeat >steps{
        repeat=steps;
    }
    let bar=("#").repeat(repeat);
    let empty=(" ").repeat(steps-repeat);
    let blank=(" ").repeat((size+1).to_string().len()-(index+1).to_string().len());
    if index==size{
        let _r=stdout().execute(Show);
    }
    else if index>0{
        let _r=stdout().execute(Hide);
    }
    println!("{}/{}{} : [{}{}]",index+1,size+1,blank,bar,empty);
    let _r=stdout().execute(MoveToPreviousLine(1));
}

fn is_allowed(spielfeld:&Vec<Vec<i32>>,row:usize,col:usize,number:i32,länge_quadrat:usize)->bool{
    let mut is_allowed=true;
    let  r=row-row%länge_quadrat;
    let c=col-col %länge_quadrat;
    for i in 0..spielfeld.len(){
        if spielfeld[i][row]==number{
            is_allowed=false;
            break;
        }
    }
    for i in 0..spielfeld[0].len(){
        if spielfeld[col][i]==number{
            is_allowed=false;
            break;
        }
    }

    for i in c..c+3{
        for d in r..r+3{
            if spielfeld[i][d]==number{
                is_allowed=false;
                break;
            }
        }
    }

    return is_allowed;
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
