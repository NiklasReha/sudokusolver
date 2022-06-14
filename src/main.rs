fn main() {
    let länge_eines_quadrats=3;
    let mut spielfeld:Vec<Vec<i32>>=vec![vec![7,9,0,0,5,8,2,0,0],
                                        vec![0,0,4,6,0,7,0,5,8],
                                        vec![5,0,3,0,0,2,6,7,0],
                                        vec![0,4,0,2,7,0,5,0,6],
                                        vec![0,3,9,5,0,0,1,8,0],
                                        vec![6,7,0,0,1,9,0,0,2],
                                        vec![9,0,0,7,0,1,0,0,4],
                                        vec![0,6,8,0,0,5,7,0,0],
                                        vec![3,0,7,4,8,0,0,2,5]];
    let mut mögliche_nummern=Vec::new();
    for i in 1..länge_eines_quadrats*länge_eines_quadrats+1{
        mögliche_nummern.push(i as i32);
    }
    
    let mut choice =false;
    let mut unresolved=0;
    loop {
        let mut gebrauchte_nummern:Vec<Vec<i32>>=Vec::new();
        let mut changed=false;
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

        for i in 0..spielfeld.len(){
            for d in 0..spielfeld[i].len(){
                if spielfeld[i][d]==0{
                    //let mut possible_solutions:Vec<i32>=Vec::new();
                    let mut possible_solutions=gebrauchte_nummern[get_index(spielfeld[0].len()/länge_eines_quadrats, d/länge_eines_quadrats,i/länge_eines_quadrats)].clone();
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
                    if possible_solutions.len() == 1 {
                        changed=true;
                        spielfeld[i][d]=possible_solutions[0];
                        break;
                    }
                    else if choice&&possible_solutions.len()>0{
                        spielfeld[i][d]=possible_solutions[0];
                    }
                }
            }
            if changed{
                break;
            }
        }
        if !changed{
            let mut solved =true;
            for i in 0..spielfeld.len(){
                for d in 0..spielfeld[i].len(){
                    if spielfeld[i][d]==0{
                        solved=false;
                    }
                }
            }
            if solved||unresolved>20{
                break;
            }
            else{
                choice=true;
            }
            unresolved+=1

        }
    }

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

fn get_index(weite:usize,momentane_weite:usize,höhe:usize)->usize{
    höhe*weite+momentane_weite
}
