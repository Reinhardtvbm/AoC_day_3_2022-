use std::fs::read_to_string;

fn main() {
    let priorities = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let file_contents = read_to_string("data.txt").unwrap();
    let lines = file_contents.lines();

    let mut elf_groups = Vec::new();

    let mut count = 0;
    let mut group = Vec::new();

    for line in lines.clone() {
        group.push(line);
        count += 1;

        if count == 3 {
            elf_groups.push(group.clone());
            count = 0;
            group.clear();
        }
    }

    let total: i32 = elf_groups
        .into_iter()
        .map(|group| {
            let mut find = 'a';

            let (group_1, group_2, group_3) = (group[0], group[1], group[2]);

            for char in group_1.chars() {
                if group_2.contains(char) && group_3.contains(char) {
                    find = char;
                    break;
                }
            }

            println!("{}", find);

            for (index, char) in priorities.iter().enumerate() {
                if *char == find {
                    return 1 + index as i32;
                }
            }

            0
        })
        .sum();

    println!("{}", total);
}
