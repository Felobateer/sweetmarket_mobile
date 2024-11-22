import 'package:flutter/material.dart';

class TitleCustom extends StatelessWidget {
  final String text;
  const TitleCustom({required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return Text(text.toUpperCase(),
        style: TextStyle(
          fontSize: 36,
          fontWeight: FontWeight.bold,
          color: Color(0xFFD97706),
          height: 1.5,
        ),
        textAlign: TextAlign.center);
  }
}
