package main

// import "fmt"

func main() {
    // result := make(chan int, 1)
    // go fib(40, result)

    basefib(40)

    // value := basefib(40)

    // value := <-result
    // fmt.Printf("Value: %d\n", value)
    // close(result)

}

func fib(n int, result chan int) {
    if (n == 1 || n == 2) {
       result <- 1
       return
    }

    a := make (chan int, 1)
    go fib ((n - 1), a)

    b := make (chan int, 1)
    go fib ((n - 2), b)

    value1 := <- a
    close(a)

    value2 := <- b
    close(b)

    result <- (value1 + value2)
    return
}


func basefib(n int) int {
    if (n == 1 || n == 2) {
        return 1
    }

    return (basefib(n - 1) + basefib(n - 2))

}
