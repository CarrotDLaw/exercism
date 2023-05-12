// define the 'expectedMinutesInOven' constant
let expectedMinutesInOven = 40

// define the 'remainingMinutesInOven' function
func remainingMinutesInOven(elapsedMinutes: Int) -> Int {
    return 40 - elapsedMinutes
}

// define the 'preparationTimeInMinutes' function
func preparationTimeInMinutes(layers: Int) -> Int {
    return layers * 2
}

// define the 'totalTimeInMinutes' function
func totalTimeInMinutes(layers: Int, elapsedMinutes: Int) -> Int {
    return preparationTimeInMinutes(layers: layers) + elapsedMinutes
}