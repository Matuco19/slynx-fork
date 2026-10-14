mod common;

/// Component methods behave like object methods: they can be instance or
/// static, use `&self`/`&mut self`, return `Self`, and read/write properties.
#[test]
fn test_component_methods() {
    common::load_example("examples/componentMethod.syx")
        .compile()
        .expect("component method example should compile");
}

/// A static method returning `Self` constructs the component with the struct
/// literal syntax `Self { .. }`.
#[test]
fn component_static_method_constructs_self() {
    common::compile_source_ok(
        r#"
component Counter {
    prop value: int;

    func new(): Self {
        Self { value: 0 }
    }
}

func main(): void {
    let counter = Counter.new();
}
"#,
    );
}

/// Instance methods read and mutate component properties through `self`.
#[test]
fn component_instance_method_accesses_props() {
    common::compile_source_ok(
        r#"
component Counter {
    prop value: int;

    func increment(&mut self, amount: int): void {
        self.value = self.value + amount;
    }

    func getValue(&self): int {
        self.value
    }
}

func main(): int {
    let mut counter = Counter { value: 1 };
    counter.increment(4);
    counter.getValue()
}
"#,
    );
}

/// Methods can call each other through the receiver.
#[test]
fn component_method_calls_method() {
    common::compile_source_ok(
        r#"
component Counter {
    prop value: int;

    func double(&self): int {
        self.value + self.value
    }

    func quadruple(&self): int {
        self.double() + self.double()
    }
}

func main(): int {
    let counter = Counter { value: 3 };
    counter.quadruple()
}
"#,
    );
}

/// A component can satisfy an interface declared in an `extend` block, and an
/// interface-constrained generic function can dispatch to it.
#[test]
fn component_implements_interface_via_extend() {
    common::compile_source_ok(
        r#"
interface Stringifiable {
    func stringify(&self) -> str
}

component Counter {
    prop value: int;
}

extend Counter: Stringifiable {
    func stringify(&self) -> str -> "counter";
}

func describe<T>(x: T): str where T: Stringifiable -> x.stringify();

func main(): void {
    let counter = Counter { value: 3 };
    describe(counter);
}
"#,
    );
}

/// Methods may take parameters and reach through nested property accesses.
#[test]
fn component_method_with_args_and_nested_props() {
    common::compile_source_ok(
        r#"
object StyleContext {
    paint: int,
}

object DrawingContext {
    scale: int,

    func render(&self, text: str, paint: int): void {
    }
}

component Text {
    prop text: str;
    prop style: StyleContext;

    func onMount(scene: int): void {
    }

    func render(&self, context: DrawingContext): void {
        context.render(self.text, self.style.paint);
    }
}

func main(): void {
    let text = Text {
        text: "hi",
        style: StyleContext(paint: 1)
    };
    Text.onMount(0);
    text.render(DrawingContext(scale: 2));
}
"#,
    );
}

/// A generic component can be instantiated with a concrete type argument and
/// one of its properties read back with the substituted type.
#[test]
fn generic_component_construct_and_access_prop() {
    common::compile_source_ok(
        r#"
component Box<T> {
    prop value: T;
}

func main(): int {
    let box = Box<int> { value: 3 };
    box.value
}
"#,
    );
}
