import 'package:dartson/dartson.dart';
import 'word_splitter.dart';
import 'word_indexer.dart';
import 'dart:io';

class Main {
  void run () async {
    var dson = new Dartson.JSON();
    var wordSplitter = new WordSplitter();
    var wordCounter = new WordFrequencyCounter();
    var contents = await new File('text/phstone.txt').readAsString();
    var words = wordCounter.wordFreq(wordSplitter.splitText(contents));

    var filteredWords = new Map<String, int>();
    void filter (k, v) {
      if (v >= 2) {
        filteredWords[k] = v;
      }
    }

    words.forEach(filter);

    var json =
        dson.encode(filteredWords);
    var outFile = new File('out/result.json');
    await outFile.writeAsString(json);
  }
}

void main () {
  var run = new Main();
  run.run();
}
