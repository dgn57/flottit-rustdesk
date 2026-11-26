// FlottIT Connect – vereinfachtes Startfenster für Kunden.
// Wird von client/apply-branding.sh nach flutter/lib/flottit/ kopiert.
//
// Zeigt: große ID zum Vorlesen, einen erklärenden Satz, Telefonnummer,
// „Beenden“ und „Über“. Keine Einstellungen, kein Verbinden-Feld.
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:flutter_hbb/common.dart';
import 'package:url_launcher/url_launcher.dart';

import 'flottit_config.dart';

/// ID in Dreiergruppen anzeigen („123 456 789“) – leichter vorzulesen.
String flottitIdFormat(String roh) {
  final ziffern = roh.replaceAll(' ', '');
  if (ziffern.isEmpty || !RegExp(r'^\d+$').hasMatch(ziffern)) return roh;
  final teile = <String>[];
  for (var i = 0; i < ziffern.length; i += 3) {
    teile.add(ziffern.substring(i, i + 3 > ziffern.length ? ziffern.length : i + 3));
  }
  return teile.join(' ');
}

class FlottitHome extends StatelessWidget {
  const FlottitHome({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final model = gFFI.serverModel;
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 10, 16, 6),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'Ihre Nummer:',
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 15, color: kFlottitGrau),
          ),
          ValueListenableBuilder<TextEditingValue>(
            valueListenable: model.serverId,
            builder: (context, wert, _) => SelectableText(
              wert.text.isEmpty ? '…' : flottitIdFormat(wert.text),
              textAlign: TextAlign.center,
              style: const TextStyle(
                fontSize: 36,
                fontWeight: FontWeight.bold,
                color: kFlottitSchwarz,
                letterSpacing: 1.5,
              ),
            ),
          ),
          const SizedBox(height: 8),
          const Text(
            'Nennen Sie diese Nummer Ihrem FlottIT-Techniker.',
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 16, color: kFlottitSchwarz),
          ),
          const SizedBox(height: 6),
          InkWell(
            onTap: () => launchUrl(Uri.parse('tel:$kFlottitTelefonLink')),
            child: const Text(
              'Telefon: $kFlottitTelefon',
              textAlign: TextAlign.center,
              style: TextStyle(
                  fontSize: 16, fontWeight: FontWeight.bold, color: kFlottitRot),
            ),
          ),
          const SizedBox(height: 14),
          SizedBox(
            height: 44,
            child: ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: kFlottitRot,
                foregroundColor: kFlottitWeiss,
                textStyle:
                    const TextStyle(fontSize: 17, fontWeight: FontWeight.bold),
              ),
              onPressed: () => flottitBeenden(),
              child: const Text('Beenden'),
            ),
          ),
          TextButton(
            onPressed: () => zeigeFlottitUeber(context),
            child: const Text('Über FlottIT Connect',
                style: TextStyle(fontSize: 13, color: kFlottitGrau)),
          ),
        ],
      ),
    );
  }
}

/// Beendet FlottIT Connect vollständig. Danach ist kein Zugriff mehr möglich.
/// Gleiches Vorgehen wie RustDesks eigener „Quit“-Knopf im Incoming-Modus
/// (desktop_home_page.dart): SystemNavigator.pop() + exit(0) unter Windows.
void flottitBeenden() {
  SystemNavigator.pop();
  exit(0);
}

Future<void> zeigeFlottitUeber(BuildContext context) async {
  String version = '';
  try {
    version = await bind.mainGetVersion();
  } catch (_) {}
  if (!context.mounted) return;
  await showDialog(
    context: context,
    builder: (ctx) => AlertDialog(
      title: Text('Über $kFlottitAppName'),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('$kFlottitAppName – Fernwartung von FlottIT'),
            Text('Version $version (Build $kFlottitBuild)'),
            const SizedBox(height: 12),
            Text('Basiert auf RustDesk $kFlottitRustdeskVersion (AGPL-3.0).'),
            const Text('© Purslane Tech Pte. Ltd. und RustDesk-Mitwirkende; '
                'Anpassungen © FlottIT.'),
            const SizedBox(height: 8),
            const Text('Quellcode:'),
            InkWell(
              onTap: () => launchUrl(Uri.parse(kFlottitQuellcode)),
              child: const Text(kFlottitQuellcode,
                  style: TextStyle(
                      color: kFlottitRot,
                      decoration: TextDecoration.underline)),
            ),
            const SizedBox(height: 8),
            const Text(
                'Dieses Programm ist freie Software. Sie dürfen es unter den '
                'Bedingungen der GNU Affero General Public License Version 3 '
                'weitergeben und verändern. Es wird OHNE JEDE GEWÄHRLEISTUNG '
                'bereitgestellt.'),
            const SizedBox(height: 8),
            InkWell(
              onTap: () => launchUrl(Uri.parse(kFlottitWebsite)),
              child: const Text('flottit.at',
                  style: TextStyle(
                      color: kFlottitRot,
                      decoration: TextDecoration.underline)),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(ctx).pop(),
          child: const Text('Schließen'),
        ),
      ],
    ),
  );
}
