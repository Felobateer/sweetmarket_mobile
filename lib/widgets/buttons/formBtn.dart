import 'package:flutter/material.dart';

typedef Action = void Function();

class Formbtn extends StatelessWidget {
  final String content;
  final Action action;
  const Formbtn({required this.content, required this.action, super.key});

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: action,
      style: ElevatedButton.styleFrom(
        backgroundColor: const Color(0xFF3B82F6),
        foregroundColor: const Color(0xFFe5e7eb),
        elevation: 4,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(30),
        ),
        minimumSize: const Size(double.infinity, 50),
        padding: const EdgeInsets.symmetric(vertical: 16.0),
      ),
      child: Text(
        content,
        style: const TextStyle(
          fontSize: 16.0,
        ),
      ),
    );
  }
}
