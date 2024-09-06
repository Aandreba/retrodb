import "models/columns.dart";
import "models/filter.dart";

final class Game {
  static const Column<Integer> id = Column("id");
}

class QueryBuilder {
  QueryFilter? where;
}
