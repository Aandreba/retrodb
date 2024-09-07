import 'dart:typed_data';
import 'package:json_annotation/json_annotation.dart';

part 'models.g.dart';

typedef Integer = int?;
typedef Real = double?;
typedef Text = String?;
typedef Blob = Uint8List?;

final class Game {
  int id;
  Text serialId;
  Text developer;
  Text publisher;
  Text rating;
  Integer users;
  Text franchise;
  Integer releaseYear;
  Integer releaseMonth;
  Text region;
  Text genre;
  Text displayName;
  Text fullName;
  Platform? platform;
  List<Rom>? roms;

  Game(this.id);
  static Game fromJson(Map<String, dynamic> json) => Game(
        (json['id'] as num).toInt(),
      )
        ..serialId = json['serial_id'] as String?
        ..developer = json['developer'] as String?
        ..publisher = json['publisher'] as String?
        ..rating = json['rating'] as String?
        ..users = (json['users'] as num?)?.toInt()
        ..franchise = json['franchise'] as String?
        ..releaseYear = (json['release_year'] as num?)?.toInt()
        ..releaseMonth = (json['release_month'] as num?)?.toInt()
        ..region = json['region'] as String?
        ..genre = json['genre'] as String?
        ..displayName = json['display_name'] as String?
        ..fullName = json['full_name'] as String?
        ..platform = _$PlatformFromJson(json);

  Uri get coverArt => getThumbnailUri("");

  Uri getThumbnailUri(String type) {
    return Uri.https("thumbnails.libretro.com", "/MAN - $platform");
  }
}

@JsonSerializable()
final class Rom {
  @JsonKey(name: "rom_id")
  int id;
  @JsonKey(name: "rom_serial_id")
  Text serialId;
  @JsonKey(name: "rom_name")
  Text name;
  @JsonKey(name: "rom_crc")
  Integer crc;

  Rom(this.id);
  static Rom fromJson(Map<String, dynamic> json) => _$RomFromJson(json);
}

@JsonSerializable()
final class Platform {
  @JsonKey(name: "platform")
  Text name;
  Text manufacturer;
}
