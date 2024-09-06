import 'dart:typed_data';
import 'filter.dart';

final class Column<T> extends QueryValue<T> {
  final String name;
  const Column(this.name);
}

// Integer
extension Integer on Column<int> {
  @override
  QueryValue<int> operator ==(int rhs) {
    return Eq(this, Literal(rhs));
  }
}
