# Hail Stone Numbers
Hail Stone numbers that is a sequince number formed from a whole unit number.

### Psedo Code
```Psedo-Code
user_input: unit64
hailstone_numbers: Vector<unit64>

hailstone_numbers.push(user_input)

function(user_input)

function(user_input) {
    match user_input % 2 {
        0 => user_input = user_input / 2
        1 => user_input = user_input * 3 + 1
    }

    hailstone_numbers.push(user_input)

    if user_input = 1 {
        print hailstone_numbers
        exit 0
    }

    function(user_input)
}
```

### Run The Program
1. install rust
2. clone the git repo
3. `cargo run`
