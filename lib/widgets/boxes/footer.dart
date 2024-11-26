import 'package:flutter/material.dart';
import 'package:sweetmarket/services/models/links.dart';

typedef Action = void Function(int index);

class Footer extends StatelessWidget {
  final int index;
  final Action nav;

  const Footer({
    required this.index,
    required this.nav,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return BottomNavigationBar(
      currentIndex: index,
      onTap: (selectedIndex) {
        nav(selectedIndex);
      },
      items: [
        for (var link in links)
          BottomNavigationBarItem(
            icon: link.icon,
            label: link.name,
          ),
      ],
      type: BottomNavigationBarType.fixed,
      fixedColor: const Color(0xFFD97706),
    );
  }
}
