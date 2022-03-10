pub struct BuildNetworkParams<'a, TBl: BlockT, TExPool, TImpQu, TCl> {
    pub config: &'a Configuration,
    pub client: Arc<TCl>,
    pub transaction_pool: Arc<TExPool>,
    pub spawn_handle: SpawnTaskHandle,
    pub import_queue: TImpQu,
    pub block_announce_validator_builder: Option<Box<dyn FnOnce(Arc<TCl>) -> Box<dyn BlockAnnounceValidator<TBl> + Send> + Send>>,
    pub warp_sync: Option<Arc<dyn WarpSyncProvider<TBl>>>,
    
    config: &'a Configuration
The service configuration.

client: Arc<TCl>
A shared client returned by new_full_parts.

transaction_pool: Arc<TExPool>
A shared transaction pool.

spawn_handle: SpawnTaskHandle
A handle for spawning tasks.

import_queue: TImpQu
An import queue.

block_announce_validator_builder: Option<Box<dyn FnOnce(Arc<TCl>) -> Box<dyn BlockAnnounceValidator<TBl> + Send> + Send>>
A block announce validator builder.

warp_sync: Option<Arc<dyn WarpSyncPro
    
    impl<'a, TBl, TExPool, TImpQu, TCl> !RefUnwindSafe for BuildNetworkParams<'a, TBl, TExPool, TImpQu, TCl>
impl<'a, TBl, TExPool, TImpQu, TCl> Send for BuildNetworkParams<'a, TBl, TExPool, TImpQu, TCl>
where
    TCl: Send + Sync,
    TExPool: Send + Sync,
    TImpQu: Send, 
impl<'a, TBl, TExPool, TImpQu, TCl> !Sync for BuildNetworkParams<'a, TBl, TExPool, TImpQu, TCl>
impl<'a, TBl, TExPool, TImpQu, TCl> Unpin for BuildNetworkParams<'a, TBl, TExPool, TImpQu, TCl>
where
    TImpQu: Unpin, 
impl<'a, TBl, TExPool, TImpQu, TCl> !UnwindSafe for BuildNetworkParams<'a, TBl, TExPool, TImpQu, TCl>
    
    impl<T> Any for T
where
    T: 'static + ?Sized, 
[src]
pub fn type_id(&self) -> TypeId
Gets the TypeId of self. Read more

[src]
impl<T> Borrow<T> for T
where
    T: ?Sized, 
[src]
pub fn borrow(&self) -> &T
Immutably borrows from an owned value. Read more

[src]
impl<T> BorrowMut<T> for T
where
    T: ?Sized, 
[src]
pub fn borrow_mut(&mut self) -> &mut T
Mutably borrows from an owned value. Read more

[src]
impl<T> CheckedConversion for T
[src]
fn checked_from<T>(t: T) -> Option<Self>
where
    Self: TryFrom<T>, 
Convert from a value of T into an equivalent instance of Option<Self>. Read more

[src]
fn checked_into<T>(self) -> Option<T>
where
    Self: TryInto<T>, 
Consume self to return Some equivalent value of Option<T>. Read more

[src]
impl<T> Downcast for T
where
    T: Any, 
[src]
pub fn into_any(self: Box<T, Global>) -> Box<dyn Any + 'static, Global>ⓘ
Convert Box<dyn Trait> (where Trait: Downcast) to Box<dyn Any>. Box<dyn Any> can then be further downcast into Box<ConcreteType> where ConcreteType implements Trait. Read more

[src]
pub fn into_any_rc(self: Rc<T>) -> Rc<dyn Any + 'static>
Convert Rc<Trait> (where Trait: Downcast) to Rc<Any>. Rc<Any> can then be further downcast into Rc<ConcreteType> where ConcreteType implements Trait. Read more

[src]
pub fn as_any(&self) -> &(dyn Any + 'static)
Convert &Trait (where Trait: Downcast) to &Any. This is needed since Rust cannot generate &Any’s vtable from &Trait’s. Read more

[src]
pub fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
Convert &mut Trait (where Trait: Downcast) to &Any. This is needed since Rust cannot generate &mut Any’s vtable from &mut Trait’s. Read more

[src]
impl<T> From<T> for T
[src]
pub fn from(t: T) -> T
Performs the conversion.

[src]
impl<T> Instrument for T
[src]
fn instrument(self, span: Span) -> Instrumented<Self>
Instruments this type with the provided Span, returning an Instrumented wrapper. Read more

[src]
fn in_current_span(self) -> Instrumented<Self>
Instruments this type with the current Span, returning an Instrumented wrapper. Read more

[src]
impl<T> Instrument for T
[src]
fn instrument(self, span: Span) -> Instrumented<Self>
Instruments this type with the provided Span, returning an Instrumented wrapper. Read more

[src]
fn in_current_span(self) -> Instrumented<Self>
Instruments this type with the current Span, returning an Instrumented wrapper. Read more

[src]
impl<T, U> Into<U> for T
where
    U: From<T>, 
[src]
pub fn into(self) -> U
Performs the conversion.

[src]
impl<T, Outer> IsWrappedBy<Outer> for T
where
    Outer: AsRef<T> + AsMut<T> + From<T>,
    T: From<Outer>, 
[src]
pub fn from_ref(outer: &Outer) -> &T
Get a reference to the inner from the outer.

[src]
pub fn from_mut(outer: &mut Outer) -> &mut T
Get a mutable reference to the inner from the outer.

[src]
impl<T> Pointable for T
[src]
pub const ALIGN: usize
The alignment of pointer.

type Init = T
The type for initializers.

[src]
pub unsafe fn init(init: <T as Pointable>::Init) -> usize
Initializes a with the given initializer. Read more

[src]
pub unsafe fn deref<'a>(ptr: usize) -> &'a T
Dereferences the given pointer. Read more

[src]
pub unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
Mutably dereferences the given pointer. Read more

[src]
pub unsafe fn drop(ptr: usize)
Drops the object pointed to by the given pointer. Read more

[src]
impl<T> Same<T> for T
type Output = T
Should always be Self

[src]
impl<T> SaturatedConversion for T
[src]
fn saturated_from<T>(t: T) -> Self
where
    Self: UniqueSaturatedFrom<T>, 
Convert from a value of T into an equivalent instance of Self. Read more

[src]
fn saturated_into<T>(self) -> T
where
    Self: UniqueSaturatedInto<T>, 
Consume self to return an equivalent value of T. Read more

[src]
impl<T, U> TryFrom<U> for T
where
    U: Into<T>, 
type Error = Infallible
The type returned in the event of a conversion error.

[src]
pub fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error>
Performs the conversion.

[src]
impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>, 
type Error = <U as TryFrom<T>>::Error
The type returned in the event of a conversion error.

[src]
pub fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>
Performs the conversion.

impl<T> Typeable for T
where
    T: Any, 
fn get_type(&self) -> TypeId
Get the TypeId of this object.

[src]
impl<S, T> UncheckedInto<T> for S
where
    T: UncheckedFrom<S>, 
[src]
pub fn unchecked_into(self) -> T
The counterpart to unchecked_from.

[src]
impl<T, S> UniqueSaturatedInto<T> for S
where
    T: Bounded,
    S: TryInto<T>, 
[src]
pub fn unique_saturated_into(self) -> T
Consume self to return an equivalent value of T.

[src]
impl<V, T> VZip<V> for T
where
    V: MultiLane<T>, 
[src]
pub fn vzip(self) -> V
[src]
impl<T> WithSubscriber for T
[src]
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>
where
    S: Into<Dispatch>, 
Attaches the provided Subscriber to this type, returning a WithDispatch wrapper. Read more

[src]
fn with_current_subscriber(self) -> WithDispatch<Self>
Attaches the current default Subscriber to this type, returning a WithDispatch wrapper. Read more

[src]
impl<T> WithSubscriber for T
[src]
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>
where
    S: Into<Dispatch>, 
Attaches the provided Subscriber to this type, returning a WithDispatch wrapper. Read more

[src]
fn with_current_subscriber(self) -> WithDispatch<Self>
Attaches the current default Subscriber to this type, returning a WithDispatch wrapper. Read more

[src]
impl<T> Erased for T
    
}
