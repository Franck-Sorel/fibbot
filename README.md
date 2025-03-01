# Readme file for FibBot.

## Description: 
  This FibBot project scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results.

## Features: 
  - 🚀 Only usefull dependencies.
  - 🌍 Re-usable in any github projects.
  - 🛠️ Based Code Writing in Rust Programming language.

# Composition:
This project own three main part. We have, the Code itself, the Action and the Workflow.

## 1 - The Action:

let's talk about the action a bit:


```sh
name: rust-fibbot-action
description: GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results

inputs:
  pr_number:
    description: "pulll request number"
    required: true
  enable_fib:
    description: "Enable Fibonacci"
    required: false
    default: "true"
  max_threshold:
    description: "Max threshold"
    required: false
    default: "100"
  github_token:
    description: "GitHub token to interact with GitHub API"
    required: true


runs:
  using: "composite"
  steps:
    - name: Build FibBot
      run: cargo build --release
      shell: bash
    - name: Run FibBot
      run: |
        export PR_NUMBER="${{ inputs.pr_number }}"
        cargo run "${{ inputs.enable_fib }}" "${{ inputs.max_threshold }}"
      env:
        GITHUB_TOKEN: ${{ inputs.github_token }}
      shell: bash
```

Let us breakdown each step of this action:

__name:__ Is the name of the action.
__description:__ a small overview of what the action does.

__inputs:__ This keyword specify the github inputs that we are goind to use for run our application. It can be an argument for you code or directly come from GitHub like (calling the repo name, the token for authentification, etc...). Under this inputs, we have all the inputs that we are going to use in for this bot, a small description of the use of each input and if it's a must when the action is trigger.

* __runs:__ let us running our application now.
* __composite:__ There is several type of action. Here we are using a composite action.
* __Steps:__ In the first step we buil our code. and in the second one we are executed it.

## 2- The workflow

```rust
name: Minimal Action Implementation

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  fibonacci:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v2

      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable #specifies the latest rust version to install

      - name: Run FibBot
        uses: Franck-Sorel/fibbot@master
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}  
        with:
          pr_number: ${{ github.event.pull_request.number }}
          enable_fib: "true"
          max_threshold: "100"
```
Let's breakdown this workflow:
__on:__ 
   *  This action is trigger by a pull request (_pull request:_ ) 

__jobs:__  
   *  Here we are definind what we are going to run in a specific environnement.

__fibonnacci:__
   * That is the name of our Jobs.

__runs-on:__
   *  Here we are defining under which Operating System we are going to run our jobs.
      
__Steps:__

  * Definition of each step that our remote environnement where our OS is running are goind to executed:
     
__name:__

   Each step has a name.
   
  * __firstly:__ we Check out our repo
  * __secondly:__ we set up the rust environnment
  * __thirly:__ we call our inputs in the environnment for our code to call it.



## Code
Our code has many files:

__add_comment.rs:__ Use to add the comment in the pull request of ouputs of each execution.

__extract_number.rs:__ Take a text of type string slice and return all the numbers inside.

__fib_number.rs__ Calculate the fibonacci of a  number.

__get_from_pull_request.rs__ In this one, we access the pull request content and take it as real text to after using it for fetch numbers.

__main:__ For the implementation of all those files.

1 and 7
