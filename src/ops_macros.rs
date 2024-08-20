// The following macros help implementing binary operators for ratiofracs
// Inspired by ratiofrac crate
#[macro_export(local_inner_macros)]
macro_rules! impl_op_ratiofrac {
	($op:ident, $method:ident $(,$requirements:ident)*) => {
		impl<T> $op<RatioFrac<T>> for &RatioFrac<T>
		where
			T: PolyxNum,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: RatioFrac<T>) -> RatioFrac<T> {
				self.$method(&other)
			}
		}

		impl<T> $op<&RatioFrac<T>> for RatioFrac<T>
		where
			T: PolyxNum,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: &RatioFrac<T>) -> RatioFrac<T> {
				(&self).$method(other)
			}
		}

		impl<T> $op<RatioFrac<T>> for RatioFrac<T>
		where
			T: PolyxNum,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: RatioFrac<T>) -> RatioFrac<T> {
				(&self).$method(&other)
			}
		}
	};
}

// The following macros allow to add ratiofracs with numbers
// Rust's specialization feature is not stable yet so we have to duplicate the
// code for each primitive type
#[macro_export(local_inner_macros)]
macro_rules! impl_op_some_primitive_ratiofrac {
	($op:ident, $method:ident, $t:ty $(,$requirements:ident)*) => {
		impl<T> $op<RatioFrac<T>> for $t
		where
			T: PolyxNum + From<$t>,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: RatioFrac<T>) -> RatioFrac<T> {
				other.$method(RatioFrac::from(std::vec![self.into()]))
			}
		}
		impl<T> $op<&RatioFrac<T>> for $t
		where
			T: PolyxNum + From<$t>,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: &RatioFrac<T>) -> RatioFrac<T> {
				other.$method(RatioFrac::from(std::vec![self.into()]))
			}
		}
		impl<T> $op<$t> for RatioFrac<T>
		where
			T: PolyxNum + From<$t>,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: $t) -> RatioFrac<T> {
				self.$method(RatioFrac::from(std::vec![other.into()]))
			}
		}
		impl<T> $op<$t> for &RatioFrac<T>
		where
			T: PolyxNum + From<$t>,
		{
			type Output = RatioFrac<T>;

			#[inline]
			fn $method(self, other: $t) -> RatioFrac<T> {
				self.$method(RatioFrac::from(std::vec![other.into()]))
			}
		}
	};
}

#[macro_export(local_inner_macros)]
macro_rules! impl_op_all_primitive_ratiofrac {
	($op:ident, $method:ident $(,$requirements:ident)*) => {
		duplicate::duplicate! {
			[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]]
		impl_op_some_primitive_ratiofrac!($op, $method, primitive_type $(,$requirements)*);
		}
	};
}

// The next macro implements the assign versions of the operators
#[macro_export(local_inner_macros)]
macro_rules! impl_assign_op_ratiofrac {
	($op:ident, $assign_op:ident, $method:ident, $assign_method: ident $(,$requirements:ident)*) => {
		impl<T> $assign_op<RatioFrac<T>> for RatioFrac<T> where T: PolyxNum
		{
			#[inline]
			fn $assign_method(&mut self, other: RatioFrac<T>) { *self = std::mem::take(self).$method(&other) }
		}
		impl<T> $assign_op<&RatioFrac<T>> for RatioFrac<T> where T: PolyxNum
		{
			#[inline]
			fn $assign_method(&mut self, other: &RatioFrac<T>) { *self = std::mem::take(self).$method(other) }
		}
		duplicate::duplicate! {
			[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]]
			impl<T> $assign_op<primitive_type> for RatioFrac<T>
			where T: PolyxNum + From<primitive_type>
			{
				#[inline]
				fn $assign_method(&mut self, other: primitive_type) { *self = std::mem::take(self).$method(RatioFrac::from(std::vec![other.into()])) }
			}
		}
	};
}

#[macro_export(local_inner_macros)]
macro_rules! impl_op_all {
	($op:ident, $assign_op:ident, $method:ident, $assign_method:ident $(,$requirements:ident)*) => {
		impl_op_ratiofrac!($op, $method $(,$requirements)*);
		impl_op_all_primitive_ratiofrac!($op, $method $(,$requirements)*);
		impl_assign_op_ratiofrac!($op, $assign_op, $method, $assign_method $(,$requirements)*);
	};
	($op:ident, $method:ident $(,$requirements:ident)*) => {
		impl_op_ratiofrac!($op, $method $(,$requirements)*);
		impl_op_all_primitive_ratiofrac!($op, $method $(,$requirements)*);
	};
}

pub(super) use impl_op_all;
