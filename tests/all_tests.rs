mod infra;

//All tests written by Sahil Ahuja except for bst test which was adapted from some incorrect GPT code


success_tests! {
    {
        name: equal, 
        file: "equal.snek",
        expected: " false\ntrue\nfalse\ntrue\n(array 10 (100 234 -24 234 -33 -56 1000 85 80 nil))\nfalse\n(array 10 (100 234 -24 234 -33 -56 1000 85 81 nil))\ntrue\ntrue\nfalse\ntrue\ntrue\nfalse\nfalse",
    }, 
    { //simple array declaration 
        name: simple_example0,
        file:  "simple_examples0.snek",
        expected:   "(array 10 (0 1 2 3 4 5 6 7 8 9))",
    },
    { //simple array declaration with less specified values than size
        name: simple_example1,
        file: "simple_examples1.snek",
        expected: "(array 10 (0 1 2 3 4 5 6 7 nil nil))"
    },
    { //testing with index method
        name: simple_example2,
        file: "simple_examples2.snek",
        expected: "4"
    },
    { //testing with index method
        name: simple_example3,
        file: "simple_examples3.snek",
        expected: "4"
    },
    { //testing with setarr method
        name: simple_example6,
        file: "simple_examples6.snek",
        expected: "(array 10 (29 1 2 3 4 5 6 7 nil nil))\n(array 10 (29 1 2 3 4 5 6 7 nil nil))\n"
    },
    { //testing with setarr method
        name: simple_example7,
        file: "simple_examples7.snek",
        expected: "(array 10 (30 1 2 3 4 5 6 7 nil nil))\n(array 10 (30 1 2 3 4 5 6 7 nil nil))\n"
    },
    { //addition of two points through a function call
        name: points,
        file: "points.snek",
        expected: "(array 2 (15 15))\n(array 2 (15 15))\n"
    },
    { //no expressions provided at instantiation
        name: simple_example8,
        file: "simple_examples8.snek",
        expected: "(array 30 (nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil))"
    },
    { //no expressions provided at instantiation with empty parentheses
        name: simple_example9,
        file: "simple_examples9.snek",
        expected: "(array 20 (nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil nil))"
    },
    /*
        A program that illustrates how your language enables the creation of binary search trees, and 
        implements functions to add an element and check if an element is in the tree.
     */
    { //creation, insertion of vals in a bst
        name: bst,
        file: "bst.snek",
        expected: "false\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\nfalse\n(array 3 (2 nil (array 3 (3 nil (array 3 (5 nil (array 3 (243 (array 3 (12 (array 3 (6 nil nil)) (array 3 (20 nil (array 3 (30 nil nil)))))) (array 3 (1000 nil nil))))))))))\n(array 3 (2 nil (array 3 (3 nil (array 3 (5 nil (array 3 (243 (array 3 (12 (array 3 (6 nil nil)) (array 3 (20 nil (array 3 (30 nil nil)))))) (array 3 (1000 nil nil))))))))))"
    },
     //testing negative values in bst
    {
        name: bst2,
        file: "bst2.snek",
        expected: "true\ntrue\ntrue\ntrue\ntrue\nfalse\nfalse\nfalse\n(array 3 (2 (array 3 (-5 (array 3 (-243 (array 3 (-1000 nil nil)) nil)) nil)) (array 3 (3 nil nil))))\nnil\nnil"
    },





    

}


runtime_error_tests! {
    { //index given greater than size
        name: error_bounds0,
        file: "error_bounds.snek",
        expected: "Runtime Error out of bounds index: 3"
    },
    { //index given less than 1
        name: error_bounds1,
        file: "error_bounds2.snek",
        expected: "Runtime Error out of bounds index: 3"
    },
    { //too many elements provided for specified size
        name: too_many_elements,
        file: "error3.snek",
        expected: "Runtime Error too many elements provided: 4"
    },
    { //checks that heap_value is actually tagged 
        name: error_tag,
        file: "error-tag.snek",
        expected: "Runtime Error tag check failed: 5"
    },
    {
        name: error3,
        file: "error3.snek",
        expected: "Runtime Error"
    }

}



static_error_tests! {


}