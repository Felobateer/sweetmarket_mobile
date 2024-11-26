import 'package:flutter/material.dart';
import 'package:sweetmarket/pages/about.dart';
import 'package:sweetmarket/pages/account.dart';
import 'package:sweetmarket/pages/home.dart';
import 'package:sweetmarket/pages/market.dart';
import 'package:sweetmarket/widgets/boxes/footer.dart';

class IndexPage extends StatefulWidget {
  const IndexPage({super.key});

  @override
  State<IndexPage> createState() => _IndexPageState();
}

class _IndexPageState extends State<IndexPage> {
  int _currentIndex = 0;

  void _onNavTap(int index) {
    setState(() {
      _currentIndex = index; // Update the current index
    });
  }

  final List<Widget> _pages = [Home(), Market(), About(), Account()];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _pages[_currentIndex],
      bottomNavigationBar: Footer(
        index: _currentIndex,
        nav: _onNavTap,
      ),
    );
  }
}
