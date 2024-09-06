import 'dart:typed_data';
import 'package:json_annotation/json_annotation.dart';

part 'models.g.dart';

typedef Integer = int?;
typedef Real = double?;
typedef Text = String?;
typedef Blob = Uint8List?;

@JsonSerializable(fieldRename: FieldRename.snake)
final class Game {
  int id;
  Text serialId;
  Integer developerId;
  Integer publisherId;
  Integer ratingId;
  Integer users;
  Integer franchiseId;
  Integer releaseYear;
  Integer releaseMonth;
  Integer regionId;
  Integer genreId;
  Text displayName;
  Text fullName;
  Integer platformId;
  Game(this.id);
}
