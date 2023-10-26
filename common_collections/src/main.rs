use std::collections::HashMap;

fn main() {
    // Common Collections

    // A vector allows you to store a variable number of values next to each other.
    // A string is a collection of characters.
    // A hash map allows you to associate a value with a particular key. It's a
    // particular implementation of the more general data structure called a map.

    vetors();

    string_in_depth();

    hash_maps();

    median();

    let numbers = vec![1, 2, 3, 4, 5];
    println!("{}", mean(&numbers));

    let numbers = [1, 2, 3, 3, 3, 7, 7, 9];

    if let Some(i) = mode(&numbers) {
        println!("The mode is {i}");
    }

    let word = pig_latin("hello");
    println!("{}", word);

    let word = pig_latin("apple");
    println!("{}", word);

    add_employee_to_department();
}

fn add_employee_to_department() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let engineering = company
        .entry(String::from("engineering"))
        .or_insert(Vec::new());

    engineering.push("sally".to_string());
    engineering.push("raven".to_string());
    engineering.push("andy".to_string());

    let sales = company.entry(String::from("sales")).or_insert(Vec::new());
    sales.push("amir".to_string());
    sales.push("kristine".to_string());

    println!("{:?}", company);

    get_all_employee_by_department("engineering", &mut company);

    fn get_all_employee_by_department(
        department: &str,
        company: &mut HashMap<String, Vec<String>>,
    ) -> () {
        if department == "engineering" {
            match company.get_mut(department) {
                Some(dep) => {
                    dep.sort();
                    println!("Sorted engineering employee: {:?}", dep);
                }
                None => {
                    println!("The department is empty!")
                }
            }
        } else {
            println!("{:?}", company.get(department));
        }
    }
}

// Convert strings to pig latin. the first consonant of each word is moved to the end
// of the word and "ay" is added, so "first" becomes "irst-fay". Words that start with
// vowel have "hay" added to the end instead "apple" becomes "apple-hay".
fn pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut new_word = String::new();

    if let Some(letter) = word.chars().nth(0) {
        let mut is_vowel = false;

        for v in vowels {
            if letter == v {
                is_vowel = true;
            }
        }

        if is_vowel {
            new_word.push_str(&format!("{}-hay", word));
        } else {
            println!("The first letter is consonant");
            for w in word.chars().skip(1) {
                new_word.push(w);
            }
            new_word.push_str(&format!("-{}ay", letter));
        }
    }

    new_word

    // for c in word.chars() {
    //     for v in vowels.chars() {
    //         if c == v {
    //             is_vowel = true;
    //             break;
    //         }
    //     }
    //     break;
    // }

    // if is_vowel {
    //     println!("the first letter is a vowel");
    // } else {
    //     println!("the first leter is consonant");
    // }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    let mut data = HashMap::new();

    for number in numbers {
        let count = data.entry(number).or_insert(0);
        *count += 1;
    }

    println!("{:?}", data);

    let mut the_mode = 0;
    for (_k, v) in data {
        if the_mode < v {
            the_mode = v;
        }
    }

    Some(the_mode)
}

fn mean(nums: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for v in nums.iter() {
        total += *v;
    }
    total
}

fn median() {
    // Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position)
    // let numbers: Vec<i32> = Vec::new();
    // let numbers = vec![7, 5, 3, 1, 9, 8];
    fn get_lowest(nums: &mut Vec<i32>) -> usize {
        let mut count = 0;
        let mut smallest_index = 0;

        while count < nums.len() {
            if nums[smallest_index] >= nums[count] {
                smallest_index = count;
            }
            count += 1;
        }

        smallest_index
    }

    fn is_odd(value: usize) -> bool {
        value % 2 == 1
    }

    fn get_median(nums: &[i32]) -> i32 {
        if is_odd(nums.len()) {
            nums[nums.len() / 2]
        } else {
            let left_middle = nums[nums.len() / 2 - 1];
            let right_middle = nums[nums.len() / 2];

            (left_middle + right_middle) / 2
        }
    }

    let mut numbers = vec![7, 5, 3, 1, 9, 8];
    let mut sorted_numbers: Vec<i32> = Vec::new();

    while 0 < numbers.len() {
        let smallest = get_lowest(&mut numbers);
        sorted_numbers.push(numbers[smallest]);
        numbers.remove(smallest);
    }

    println!("{:?}", &sorted_numbers);

    let median = get_median(&sorted_numbers);

    println!("The median is {median}");
}

fn hash_maps() {
    // The last of our common collections is the hash map. The type HashMap<K, V>
    // stores a mapping of keys of type K to values of type V using a hasing function,
    // which determines how it places these keys and values into memory.

    // Creating a new hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let mut check: HashMap<i32, i32> = HashMap::new();
    check.insert(0, 1);
    check.insert(1, 4);

    for v in check {
        println!("the value is : {}", v.1);
    }

    // Accessing values in a hash map
    let team_name = String::from("None");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} team score is: {score}");
    // Here, score will have the value that's associated with the Blue team, and
    // the result will be 10. The get method returns an Option<&V>; if there's no
    // value for that key in the hash map, get will return None. This program handles
    // the Option by calling copied to get an Option<i32> rather than an Option<&i32>,
    // then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Updating a hash map
    // Although the number of key and value pairs is growable, each unique key can
    // only have one value associated with it at a time.

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // will print {"Blue": 25}. The original value of 10 has been
                              // overwritten

    // Adding a key and value only if a key isn't prensent
    scores.entry(String::from("Yello")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // The split_whitespace method returns an iterator over sub-slices, separated
    // by whitespace, of the value in text. The or_insert method returns a mutable
    // reference (&mut V) to the value for the specified key. Here we store that
    // mutable reference in the count variable, so in order to assign to that value
    // we must first dereference count using the asterisk(*). The mutable reference
    // goes out of scope at then end of the for loop, so all of these changes are
    // safe allowed by the borrowing rules.

    // Hasing Functions
    // By default, HashMap uses a hashing function called SipHash that can provide
    // resistance to Denial of Service (DoS) attacks involving hash tables. This is not
    // the fastest hashing algorithm available but the trade-off for better security
    // that comes with the drop in performance is worth it. If you profile your code
    // and find that the default hash function is too slow for you purposes, you can
    // switch to another function by specifying a different hasher. A hasher is a type
    // that implements the BuildHasher trait.
}

fn string_in_depth() {
    // Storing UTF-8 Encoded Text witth Strings

    // We discuss strings in the con<F10>text of collections because string are implemented
    // as a collection of bytes, plus some methods to provide useful functionality when
    // those bytes are interpreted as a text.

    // What is a String?
    // When rustaceans refer to "strings" in Rust, they might be referring to either the String
    // or the string slice &str types, not just one of those types.

    // Creating a New String
    // Many of  the same operations available with Vec<T> are available with String as well
    // because String is actually implemented as a wrapper arounde vector of bytes with
    // some extra guarantees, restrictions, and capabilities.

    let mut s = String::new();
    s.push_str("hello from String");
    println!("{}", s);

    // Updating a String
    // A String can grow in size and its contents can change, just like the contents
    // of a Vec<T>, if you push more data into it. You can conveniently use the +
    // operator or the format! macro to concatenate String values.

    let mut s1 = String::from("foo");
    s1.push_str("bar");
    s1.push('F');
    s1.push('I');
    s1.push('Z');
    println!("{}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // The string s3 will contain Hello, world!. The reason s1 is no longer valid
    // after addition, and the reason we used a reference to s2, has to do with the
    // signature of the method that's called when we use the + operator. The +
    // operator uses the add method, whose signature looks something like this:

    // fn add(self, s: &str) -> String {}
    // The reason we're able to use &s2 in the call to add is that the compiler can
    // coerce the &String argument into a &str. When we call the add method, Rust
    // uses deref coercion, which here turns &s2 into &s2[..].

    // Second, we can see in the signature that add takes onwership of self, because
    // self does not have an &. This means s1 will be moved into the add call and
    // will no longer be valid after that. So although let s3 = s1 + &s2; looks like
    // it will copy both strings and create a new one, this statement actually takes
    // ownership of s1, appends to copy of the contents of s2, and then return ownership
    // of the result. In other words, it looks like it's making a lot of copies
    // but isn't; the implementation is more efficient than copying;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // At this point, s will be tic-tac-toe. With all of the + and " characters,
    // it's difficult to see what's going on. For more complicated string
    // combining, we can instead use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    // format! macro uses references so that this call doesn't take ownership
    // any of its parameter

    // Indexing into Strings
    // In many other programming languages, accessing individual characters in a
    // string by referencing them by index is valid and common operation. However,
    // if you try to access parts of a String using indexing syntax in Rust, you'll
    // get an error.
    // let s1 = String::from("hello");
    // let h = s1[0];

    // Internal Representation
    // A String is a wrapper over a Vec<u8>. Let's look at some of our property
    // encoded UTF-8
    let hello = String::from("hola");

    // In this case, len will  be 4, which means the vector storing the string
    // "hola" is 4 bytes long. Each of these letters takes 1 byte when encoded
    // in UTF-8. The following line, however, may surprise you.

    let hello = String::from("Здравствуйте");

    // Asked how long the string is, you might say 12. In fact, Rust's answer is
    // 24: that's the number of bytes it takes to encode "Здравствуйте" in UTF-8,
    // because each Unicode scalar value in that string takes 2 bytes of storage.
    // Therefore, an index into the string's bytes will not always correlate to
    // valid Unicode scalar value. To demonstrate, consider this invalid Rust code:

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // You already know that answer will not be 3, the first letter. When encoded
    // in UTF-8, the first byte of 3 is 208 and the second is 151, so it would
    // seem that answer should if fact be 208, but 208 is not a valid character
    // ot its own. Returning 208 is likely not what a user would want if they
    // asked for the first letter of this string; however, that's the only data
    // that Rust has at byte index 0. Users generally don't want the byte value
    // returned, even if the string contains only Latin letters; if &"hello"[0]
    // were valid code that returned the byte value, it would return 104, not h.

    // The answer, then, is that to avoid returning an unexpected value and causing
    // bugs that might not be discovered immediately, Rust doesn't compile this
    // code at all and prevents misunderstandings early in the development process.

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!
    // Another point about UTF-8 is that there are actually three relevant ways to
    // look at strings from Rust's perspective: as bytes, scalar values, and grapheme
    // cluster (the closest thing to what we would call letters).

    // If we look at the hind word “नमस्ते” written in the Devanagari script, it is
    // stored as a vector of u8 values that looks like this:
    // [
    //     224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    // ]
    // That's 18 bytes and is how computer ultimately store this data. If we look
    // at them as Unicode scalar values, which are what Rust's char type is, those
    // bytes look like this:
    // ['न', 'म', 'स', '्', 'त', 'े']
    // There are six char values here, but the fourth and six are not letters;
    // they're diacritics that don't make sense on their own. Finally, if we look
    // at them as grapheme clusters, we'd get what a person would call the four
    // letters that make up the hindi word:
    // ["न", "म", "स्", "ते"]

    // Rust provides different ways of interpreting the raw string data that
    // computers store so that each program can choose the interpretation it needs,
    // no matter what human language the data is in.

    // A final reason Rust doesn't allow us to index into a String to get a character
    // is that indexing operation are expeced to always constant time (O(1)). But
    // it isn't possible to guarantee that performance with a String, because
    // Rust would have to walk through the contents from the beginning to the
    // index to determine how many valid characters there were.

    // Slicing Strings
    // Indexing into a string is often a bad idea because it's not clear what the
    // return type of the string-indexing operation should be: a byte value, a character,
    // a grapheme cluster, or a string slice. If you really need to use indices to
    // create string slices, therefore, Rust asks you to be more specific.

    // Rather than indexing using [] with a single number, you can use [] with a
    // range to create a string slice containing particular bytes:
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // Here, s will ne a &str that contains the first 4 bytes of the string. Earlier,
    // we mentioned that each of these characters was 2 bytes, which means s will be Зд.
    // If we were to try to slice only part of a character's bytes with something like
    // &hello[0..1], Rust would panic at runtime in the same way as if an invalid index
    // were accessed in a vector:
    // You should use ranges to create string slices with caution, because doing so
    // can crash your program.

    // Method for Iterating Over Strings
    // The best way to operate on pieces of strings is to be explicit about wether
    // you want characters or bytes. For individual Unicode scalar values, use the
    // chars method. Calling chars on “Зд” separates out and returns two values of
    // type char,
    for c in "Зд".chars() {
        println!("{c}");
    }

    // Alternatively, the bytes method returns each raw byte, which might be appropriate
    // for you domain:
    // But be sure to remember that valid Unicode scalar values may be made up of
    // more than 1 byte.
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Getting grapheme clusters from strings as with the Devanagari script is complex,
    // so this functionality is not provided by the standard library.

    // Strings are not so simple

    let name = String::from("raven paragas");
    println!("{}", name.contains("gas"));
    let new = name.replace("raven", "kristine");
    println!("old: {}, new: {}", name, new);
}

#[allow(unused_variables)]
fn vetors() {
    // Create a new empty vector.
    let v: Vec<i32> = Vec::new();

    // Create a vector with initial values using vec! macro
    // Rust can infer the type of v
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element."),
    }

    if let Some(i) = third {
        println!("third element is: {i}");
    }

    // Iterating over the values in Vector

    // for loop to get immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // for loop to get mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * is use to dereference to get the value in i before we can use the += operator
        *i += 50;
    }
    println!("{:?}", v);

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let first = row.get(0);

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("The value is: {}", i),
        SpreadsheetCell::Float(i) => println!("The value is: {}", i),
        SpreadsheetCell::Text(i) => println!("The value is: {}", i),
    }

    // println!("{:?}", row);

    // Dropping a Vector Drops its Elements
    {
        let v = vec![1, 2, 3, 4];
    } // v goes out of scope and is freed here
}
