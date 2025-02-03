# Project info

Platform:   Zenva

Class:      Learn Rust Programming for Beginners

Section:    23, Functions

# Summary

This discusses the basics of functions.

Below is just a bunch of notes I had been taking regarding functions, based on how it was presented in the class on Zenva.

# What is a function?

A function is commonly seen like this: `function()`. Each program has a `main()` function.

You can write, `main()`, `main function`, or `main` - they all mean the same thing.

## Why use a function?

There are two primary reasons for using a `function()`.

1. **Readability**: using functions helps you break programs down into "logic blocks" of sorts. It helps to make something more "digestible". If you had a program of thousands of lines, and it was not split into different functions, it would be incredibly hard to read effectively and quickly by people who are not familiar with the program.
2. **Mathematics & Reuse**: In math, after a function is learned, it can easily be reused in the future. Therefore, it is reuseable. Writing code which is reuseable is a huge benefit.

# Readability

## Example 1 (Without Functions)

Lorem ipsum odor amet, consectetuer adipiscing elit. Aenean proin vivamus lobortis suscipit metus, diam cras rhoncus. Curabitur sapien volutpat vulputate fermentum; scelerisque massa ultrices. Vivamus cubilia orci est; tortor praesent arcu interdum. Habitasse ad senectus vulputate feugiat ex mattis commodo nisl. Curae faucibus dictum iaculis velit; ad nulla dui. Magnis laoreet elit accumsan maximus vestibulum vulputate proin. Himenaeos sed tellus diam nibh, montes eu dictum. Aliquet himenaeos ex aptent cubilia lobortis aenean montes. Risus laoreet praesent tincidunt bibendum dignissim sollicitudin interdum. Dui justo sem varius faucibus nostra velit eget. Purus curabitur habitasse fringilla blandit duis aptent donec habitasse. Felis morbi finibus proin rhoncus quisque, litora ex pretium nullam. Curae semper ornare lorem metus platea. Consectetur ullamcorper lacinia libero fermentum rutrum malesuada. Maecenas penatibus sit adipiscing sit ligula facilisis per curae. Parturient feugiat bibendum eu mauris bibendum laoreet habitant porttitor. Fermentum mauris dictum lacus cras potenti congue. Eleifend condimentum ornare vivamus penatibus tellus aptent eget.Lorem ipsum odor amet, consectetuer adipiscing elit. Aenean proin vivamus lobortis suscipit metus, diam cras rhoncus. Curabitur sapien volutpat vulputate fermentum; scelerisque massa ultrices. Vivamus cubilia orci est; tortor praesent arcu interdum. Habitasse ad senectus vulputate feugiat ex mattis commodo nisl. Curae faucibus dictum iaculis velit; ad nulla dui. Magnis laoreet elit accumsan maximus vestibulum vulputate proin. Himenaeos sed tellus diam nibh, montes eu dictum. Aliquet himenaeos ex aptent cubilia lobortis aenean montes. Risus laoreet praesent tincidunt bibendum dignissim sollicitudin interdum. Dui justo sem varius faucibus nostra velit eget. Purus curabitur habitasse fringilla blandit duis aptent donec habitasse. Felis morbi finibus proin rhoncus quisque, litora ex pretium nullam. Curae semper ornare lorem metus platea. Consectetur ullamcorper lacinia libero fermentum rutrum malesuada. Maecenas penatibus sit adipiscing sit ligula facilisis per curae. Parturient feugiat bibendum eu mauris bibendum laoreet habitant porttitor. Fermentum mauris dictum lacus cras potenti congue. Eleifend condimentum ornare vivamus penatibus tellus aptent eget.

## Example 2 (With Functions)

**Lorem ipsum**

Odor amet, consectetuer adipiscing elit. Aenean proin vivamus lobortis suscipit metus, diam cras rhoncus. Curabitur sapien volutpat vulputate fermentum; scelerisque massa ultrices. Vivamus cubilia orci est; tortor praesent arcu interdum.

Curae semper ornare lorem metus platea. Consectetur ullamcorper lacinia libero fermentum rutrum malesuada.

**Habitasse ad senectus**

Vulputate feugiat ex mattis commodo nisl. Curae faucibus dictum iaculis velit; ad nulla dui. Magnis laoreet elit accumsan maximus vestibulum vulputate proin.

Eleifend condimentum ornare vivamus penatibus tellus aptent eget.

**Himenaeos sed tellus diam nibh**

Montes eu dictum. Aliquet himenaeos ex aptent cubilia lobortis aenean montes. Risus laoreet praesent tincidunt bibendum dignissim sollicitudin interdum. Dui justo sem varius faucibus nostra velit eget. Purus curabitur habitasse fringilla blandit duis aptent donec habitasse.

Felis morbi finibus proin rhoncus quisque, litora ex pretium nullam.

**Maecenas penatibus**

Parturient feugiat bibendum eu mauris bibendum laoreet habitant porttitor. Fermentum mauris dictum lacus cras potenti congue.


# Mathematics & Reuseability

`f(x,y)=2x+y+3`

Can be applied to both:

`f(2,3)`=10

and

`f(3,5)`=14

**Note:** I **only** highlighted the first part, not the =sum. This is on purpose. That's closer to how a function looks in Rust.

# Making a `function`

The jargon for making a `function` is called `function defintion`. A `funtion definition` will go below `main()`, and look something like this:

```
fn my_amazing_function() {
    println!("This is an amazing function that does amazing stuff!");
}
```

# Using a `function`

After making a `function`, you can call it in `main()`, like so:

```
fn main() {
    my_amazing_function_name();
}
```

# Passing a `parameter` to a function

Passing a `parameter` to a function is what helps a function be more *dynamic*.

In order to pass a `parameter`, you write a variable within the parathesis, followed by the `datatype`. You will then reference, or "pass" that variable to the body of the function.

**Note:** It is **crucial** that you provide a `datatype` to the `parameter`!

## Example

```
fn my_amazing_function_name(name:String) {
    println!("Hi {}, this is an amazing function that does amazing stuff", name);
}
```

The `parameter` is `name`. The `datatype` is `String`. The `parameter` named `name` was then "passed" to `println`.

**Note:** So far, this is not doing anything yet. Things have been defined, but it's not activly "doing stuff". This is just showing how to set things up.

# Pass a `parameter` to a different `function`

Suppose that you want `name` sent to `main()`. To do so, you would want to "pass" the `value` from your `my_amazing_function_name()` to `main()` like so:

## Example

```
fn main() {
    let racoons_smell_bad = String::from("Johnny");
    my_amazing_function_name(racoons_smell_bad);
}

fn my_amazing_function_name(name:String) {
    println!("Hi {}, this is an amazing function that does amazing stuff", name);
}
```

In this example, the `parameter` that we used in our function, `name`, does *not* appear inside of `main()`. This is to show that you don't *have* to use the same `parameter` name when calling a `function` somewhere else. I can pass the content of the println `my_amazing_function_name()` to `main()` using whatever `parameter` name I want.

Keep in mind, the parameter `name` did *not* have any `value` inside of it when it was passed to `main()`. We defined the `value` of `racoons_smell_bad` only in `main()`! This is important.

# `pass` vs `return`

## Before continuing, it is important to understand that the word `pass` and `return` are **two different things**

The way I understand it is this:
* If a value is `passed`, it is either: "made", or "carried over" from some other thing.
* If a value is `returned`, it is the **outcome**, **result of**, or mathematical **solution to**...however many `variables` being calculated together.

## Step-by-step example of `pass` vs `return`
1. `a` and `b` are `*PASSED*` to `some_function()`
   1. Example:

    ```
   fn some_function(a:i32, b:i32) {
   }
    ```
   2. In this function, both `a` and `b` **DO NOT** have a `value`! All they have is a `datatype`.
2. `some_function()` adds `a` and `b` together
   1. Example:

    ```
   fn some_function(a:i32, b:i32) {
        let sum=a+b;
   }
    ```
3. `some_function()` will `*RETURN*` the outcome of `sum` to `main()`.
   1. Example:
    ```
       fn some_function(a:i32, b:i32) -> i32 {
            let sum=a+b;
            sum
       }
    ```

**Note:** That is *exactly* how `sum` would be written in order to `return` the value of `sum`. This will be explained below!

### TL;DR

* `pass` = make/carry
* `return` = result of math

# Making a `function` which `returns` a `value`

With the function we made previously, `my_amazing_function_name()`, it `passed` a `parameter` to `main()` and gave it a `value`, but we did NOT `return` a `value` to `main()`.

In order to `return` a `value`, it must meet some criteria:
* Does NOT end with a `semicolon`
* Is at the END of the `function()` (...?)
  * Unclear if this second one is correct, or if the instructor was speaking generally

## Example

```
fn main() {
    let sum = calculate_sum(5,10);
    println!("The value of calculate_sum is: {}", sum);
}

fn calculate_sum(a:i32, b:i32) -> i32 {
    let sum = a+b;
    sum
}
```

The things you should take note of are this (I will be jumping around):
* `-> i32`
  * This says, "A `value` will be `returned` inside of this `function`. That `value` is (whatever) `datatype`."
* `sum`
  * **Note:** Pay special attention to the fact that there is **no semicolon**.
  
    The lack of a semicolon makes this a special type of thing, called an *expression*. Any time a variable does not have a semicolon, it is an *expression*.
* `let sum = calculate_sum(5,10);`
  * This line `calls` the function `calculate_sum()` and it applies the `values` of `5` and `10` to parameters `a` and `b`
* `println!("The value of calculate_sum is: {}", sum);`
  * This prints the value of `sum`.
  * Again, this didn't *have* to be called `sum`. It just as easily could have been `dumpster_fire`.

# `expression` vs `statement`

The instructor mentioned this towards the end. The only things I know about them right now are this:
* A `statement` ends *with* a `semicolon`
* A `statement` does *not* `return` a `value`

## Example of a `statement`

```
let mut example = 1;
```

* An `expression` ends *without* a `semicolon`
* An `expression` *does* `return` a `value`

## Example of an `expression`

```
sum
```

