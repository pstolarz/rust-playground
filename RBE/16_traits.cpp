#include <string>
#include <iostream>

using namespace std;

template<typename Self>
struct Animal
{
    // Associated function signature; `Self` refers to the implementor type.
    static Self create(const string& name);

    // Method signatures; these will return a string.
    virtual const string& get_name() = 0;
    virtual const string& get_noise() = 0;

    // Traits can provide default method definitions.
    virtual void talk() {
        cout << this->get_name() << " says " << this->get_noise() << endl;
    }
};

struct Sheep: Animal<Sheep>
{
    bool is_naked() {
        return this->naked;
    }

    void shear() {
        if (is_naked()) {
            // Implementor methods can use the implementor's trait methods.
            cout << this->get_name() << " is already naked...\n";
        } else {
            cout << this->name << " gets a haircut!\n";

            this->naked = true;
        }
    }

    // Implement the `Animal` trait for `Sheep`
    static Sheep create(const string& name) {
        return Sheep{name, false};
    }

    const string& get_name() override {
        return this->name;
    }

    const string& get_noise() override {
        if (this->is_naked()) {
            return Sheep::noise_naked;
        } else {
            return Sheep::noise;
        }
    }

    // Default trait methods can be overridden.
    void talk() override {
        // For example, we can add some quiet contemplation.
        cout << this->name << " pauses briefly... " << this->get_noise() << endl;
    }

    bool naked;
    const string& name;

    static const string noise;
    static const string noise_naked;

private:
    Sheep(const string& name, bool naked):
        name(name), naked(naked) {}
};

const string Sheep::noise = string{"baaaaah?"};
const string Sheep::noise_naked = string{"baaaaah!"};

int main(void)
{
    auto dolly = Sheep::create("Dolly");

    dolly.talk(); 
    dolly.shear();
    dolly.talk();

    return 0;
}
