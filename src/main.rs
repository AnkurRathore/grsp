fn main() {
    let ctx_lines = 2;
    let search_term = "strong";
    let quote = "\
If you set yourself to your present task along the path of true reason, with all determination, 
vigour, and good will: if you admit no distraction, but keep your own divinity pure and 
standing strong, as if you had to surrender it right now; if you grapple this to you, 
expecting nothing, shirking nothing, but self-content with each present action taken in 
accordance with nature and a heroic truthfulness in all that you say and mean 
then you will lead a good life. And nobody is able to stop you";

//tags holds line numbers where there is a match
let mut tags: Vec<usize> = vec![];
//this will contain a vector per match to hold context lines
let mut ctx: Vec<Vec<(usize, String)>> = vec![];
for (i ,line) in quote.lines().enumerate(){
        if line.contains(search_term){
            tags.push(i);

            let v = Vec::with_capacity(2*ctx_lines + 1);
            ctx.push(v);
            
        }
    }

    //if no match, exit early
    if tags.is_empty(){
        return;
    }

    for (i, line) in quote.lines().enumerate(){
        for (j, tag) in tags.iter().enumerate(){
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i<= upper_bound){
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter(){
        for &(i, ref line) in local_ctx.iter(){
            let line_num = i + 1;
            println!("{}: {}", line_num, line)
        }
    }
}
