

```text
    (state, event) -> state
```

```text
    (Aggregate, Command) -> Vec<NewEvent>
```
```text
    (NewEvent, Store) -> DomainEvent
```
```text
    (Aggregate(n), DomainEvent) -> Aggregate(n+1)
```


## Dispatch a Command

Given a command we need to find the right aggregate instance to handle it.

When dispatcher gets the aggregate instance it calls the handle_command function. The returned
list of events (`Vec<NewEvent>`) will be enriched with the address properties of the aggregate to
get the related `DomainEvent`.

```rust
    trait DispatchCommand<C, S> {
        type Aggregate: HandleCommand<C, Self> + WithAggregateId;
        type Error;

        fn dispatch_command(command: C, context: S) -> Result<Vec<DomainEvent<E, AggregateIdOf<Self::Aggregate>>>, Self::Error>;
    }
```  
