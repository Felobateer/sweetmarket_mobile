import 'package:flutter/material.dart';

typedef Action = void Function();

class Colorbtn extends StatelessWidget {
  final String text; // Button text
  final Color color; // Button background color
  final Action action; // Button action

  const Colorbtn({
    required this.text,
    required this.color,
    required this.action,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: action,
      style: ElevatedButton.styleFrom(
        backgroundColor: color,
        foregroundColor: Colors.white,
        elevation: 5,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(30),
        ),
        padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 20),
        textStyle: const TextStyle(
          fontSize: 20,
          fontWeight: FontWeight.bold,
        ),
      ),
      child: Text(text),
    );
  }
}
