// AUTOGENERATED FILE - DO NOT EDIT

import 'public.external.dart';

class Post {
  String slug;
  String title;
  String body;
  String author;

  Post({
    required this.slug,
    required this.title,
    required this.body,
    required this.author,
  });

  Map<String, dynamic> toJson() => $PostToJson(this);
  factory Post.fromJson(Map<String, dynamic> json) => $PostFromJson(json);
}

Map<String, dynamic> $PostToJson(Post instance) => <String, dynamic>{'slug':instance.slug,'title':instance.title,'body':instance.body,'author':instance.author};

Post $PostFromJson(Map<String,dynamic>json)=>Post(slug:json['slug'] as String,title:json['title'] as String,body:json['body'] as String,author:json['author'] as String);