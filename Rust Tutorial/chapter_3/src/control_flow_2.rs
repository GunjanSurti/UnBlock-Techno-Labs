pub fn flow_2() {
    // You can optionally specify a "loop label" on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote

    let mut ctr = 0;
    'counting_up: loop { // labelling loop using ' single quote
        println!("count = {ctr}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if ctr == 2 {
                break 'counting_up; // here we are breaking the main loop by providing lable
            }
            remaining -= 1;
        }
        ctr += 1;
    }
    println!("End count = {ctr}");
    // The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first break that doesn’t specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop

        let mut no = 3;
        while no != 0 {
            println!("{no}...");
            no -= 1;
        }
        println!("LFGHO!!!");
}