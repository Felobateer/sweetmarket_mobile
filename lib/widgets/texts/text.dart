import 'package:flutter/material.dart';

class customText extends StatelessWidget {
  final String text;
  final Color color;
  const customText({required this.color, required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: TextStyle(
        fontSize: 16,
        fontFamily: 'Poppins',
        color: color,
        height: 1.1,
      ),
    );
  }
}
