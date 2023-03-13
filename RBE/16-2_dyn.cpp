#include <string>
#include <iostream>

using namespace std;

struct Animal_dyn {
    // Instance method signature
    virtual const string& noise() = 0;
};

// Implement the `Animal` trait for `Sheep`.
struct Sheep_dyn: Animal_dyn {
    const string& noise() override {
        return _noise;
    }
private:
    static const string _noise;
};
const string Sheep_dyn::_noise{"baaaaah!"};

// Implement the `Animal` trait for `Cow`.
struct Cow_dyn: Animal_dyn {
    const string& noise() override {
        return _noise;
    }
private:
    static const string _noise;
};
const string Cow_dyn::_noise{"moooooo!"};

// Returns some struct that implements Animal, but we don't know which one at compile time.
Animal_dyn* random_animal(double random_number) {
    if (random_number < 0.5) {
        return new Sheep_dyn{};
    } else {
        return new Cow_dyn{};
    }
}

int main() {
    double random_number = 0.234;
    auto animal = random_animal(random_number);
    cout << "You've randomly chosen an animal, and it says " <<  animal->noise() << endl;
}
