fn main() {
    let länge_eines_quadrats=3;
    let mut solved =true;
    let mut spielfeld:Vec<Vec<i32>>=vec![vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0],
                                        vec![0,0,0,0,0,0,0,0,0]];
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






        print!("Starting to guess...");
        let mut guess=0;
        while guess<länge_eines_quadrats*länge_eines_quadrats{
            if solutionguesser(spielfeld[0].len(), spielfeld.len(), 0, 0, &spielfeld, &mögliche_nummern, länge_eines_quadrats,guess){
                break;
            }
                guess+=1;
        }
    }
}

fn solutionguesser(weite:usize,höhe:usize,position_weite:usize,position_höhe:usize,spielfeld:&Vec<Vec<i32>>,mögliche_nummern:&Vec<i32>,länge_eines_quadrats:usize,guess:usize)->bool{
    let mut spielfeld_übergabe=spielfeld.clone();
    let mut lastguess:usize=0;
    let maxindex=get_index(weite, weite, höhe);
    let gesamtindex=get_index(weite, position_weite, position_höhe);
    print!("\nTiefe: {}",gesamtindex);
    let coords_to_call=split_index(gesamtindex+1, weite);
    let gebrauchte_nummern=get_gebrauchte_nummern(mögliche_nummern, spielfeld, länge_eines_quadrats);
    println!("\n{}\n",maxindex);

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







    let possible_solutions=get_possible_numbers(gebrauchte_nummern[get_index(spielfeld[0].len()/länge_eines_quadrats, position_weite/länge_eines_quadrats,position_höhe/länge_eines_quadrats)].clone(), spielfeld, position_höhe, position_weite);
    if possible_solutions.len()==0{
        return false;
    }
    if spielfeld[position_höhe][position_weite]!=0&& gesamtindex<=maxindex{
        loop{
            if lastguess <länge_eines_quadrats*länge_eines_quadrats&&gesamtindex+1<maxindex{
                if solutionguesser(weite, höhe, coords_to_call.1, coords_to_call.0, spielfeld, mögliche_nummern, länge_eines_quadrats, lastguess){
                    return true;
                }
            }
            else {
                break;
            }
            lastguess+=1
        }
        return false;
    }
    else if maxindex==gesamtindex{
        if possible_solutions.len()==0{
            return false;
        }
        else if guess<possible_solutions.len(){
            spielfeld_übergabe[position_höhe][position_weite]=possible_solutions[guess];
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
            return true;
        }
        else{
            return false;
        }
    }
    else{
        if possible_solutions.len() == 0||guess >= possible_solutions.len(){
            return false;
        }
        else{
            spielfeld_übergabe[position_höhe][position_weite]=possible_solutions[guess];
            loop{
                if lastguess <länge_eines_quadrats*länge_eines_quadrats&&gesamtindex+1<maxindex{
                    if solutionguesser(weite, höhe, position_weite, position_höhe, &spielfeld_übergabe, mögliche_nummern, länge_eines_quadrats, lastguess){
                        return true;
                    }
                }
                else {
                    break;
                }
                lastguess+=1
            }
            return false;
        }
    }

}

fn split_index(gesamtindex:usize,weite:usize)->(usize,usize){
    let hoehe = gesamtindex/weite;
    (hoehe,gesamtindex-hoehe)
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
