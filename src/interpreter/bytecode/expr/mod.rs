pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

mod unop_neg;
mod unop_not;
mod unop_len;

mod binop_access;
mod binop_index;
mod binop_mul;
mod binop_div;
mod binop_mod;
mod binop_add;
mod binop_sub;
mod binop_concat;
mod binop_lt;
mod binop_lte;
mod binop_gt;
mod binop_gte;
mod binop_eq;
mod binop_ne;
mod binop_and;
mod binop_or;