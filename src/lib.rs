trait IDoSomethingSync {
    fn do_something(&self);
}

struct DecoratorSync {
    delegate: Box<dyn IDoSomethingSync>,
}

impl IDoSomethingSync for DecoratorSync {
    fn do_something(&self) {
        println!("decorating");
        self.delegate.do_something();
    }
}

#[cfg(test)]
mod test_sync {
    use super::*;

    #[test]
    fn happy_case() {
        struct Delegate {}
        impl IDoSomethingSync for Delegate {
            fn do_something(&self) {
                println!("I am decorated");
            }
        }
        let sut = DecoratorSync {
            delegate: Box::new(Delegate {}),
        };
        sut.do_something();
    }
}

/// The same decorator but now for async
use async_trait::async_trait;
#[async_trait]
trait IDoSomethingASync: Send + Sync {
    async fn do_something(&self);
}

struct DecoratorASync {
    delegate: Box<dyn IDoSomethingASync + Send + Sync>,
}

#[async_trait]
impl IDoSomethingASync for DecoratorASync {
    async fn do_something(&self) {
        println!("decorating");
        self.delegate.do_something().await;
    }
}

#[cfg(test)]
mod test_async {
    use super::*;

    #[tokio::test]
    async fn happy_case() {
        struct Delegate {}
        #[async_trait]
        impl IDoSomethingASync for Delegate {
            async fn do_something(&self) {
                println!("I am decorated");
            }
        }
        let sut = DecoratorASync {
            delegate: Box::new(Delegate {}),
        };
        sut.do_something().await;
    }
}
