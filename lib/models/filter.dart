import "columns.dart";

sealed class QueryValue<T> {
  const QueryValue();
}

final class Literal<T> extends QueryValue<T> {
  final T value;
  Literal(this.value);
}

// Equality
final class Eq<T, Lhs extends QueryValue<T>, Rhs extends QueryValue<T>>
    extends QueryValue<T> {
  final Lhs lhs;
  final Rhs rhs;
  Eq(this.lhs, this.rhs);
}
