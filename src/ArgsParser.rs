# package VASSAL.tools;

import java.util.Properties;

public class ArgsParser {
  private final Properties props;

  public ArgsParser(String[] args) {
    props = new Properties();
    for (int i = 0; i < args.length; ++i) {
      if (args[i].startsWith("-")) {
        if (i < args.length - 1
          && !args[i + 1].startsWith("-")) {
          props.put(args[i].substring(1), args[++i]);
        }
        else {
          props.put(args[i].substring(1), "");
        }
      }
    }
  }

  public Properties getProperties() {
    return props;
  }
}
