// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'models.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Rom _$RomFromJson(Map<String, dynamic> json) => Rom(
      (json['rom_id'] as num).toInt(),
    )
      ..serialId = json['rom_serial_id'] as String?
      ..name = json['rom_name'] as String?
      ..crc = (json['rom_crc'] as num?)?.toInt();

Map<String, dynamic> _$RomToJson(Rom instance) => <String, dynamic>{
      'rom_id': instance.id,
      'rom_serial_id': instance.serialId,
      'rom_name': instance.name,
      'rom_crc': instance.crc,
    };

Platform _$PlatformFromJson(Map<String, dynamic> json) => Platform()
  ..name = json['platform'] as String?
  ..manufacturer = json['manufacturer'] as String?;

Map<String, dynamic> _$PlatformToJson(Platform instance) => <String, dynamic>{
      'platform': instance.name,
      'manufacturer': instance.manufacturer,
    };
