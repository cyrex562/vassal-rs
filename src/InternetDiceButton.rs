/*
 *
 * Copyright (c) 2003 by Brent Easton and Rodney Kinney
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License (LGPL) as published by the Free Software Foundation.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, copies are available
 * at http://www.opensource.org.
 */
# package VASSAL.build.module;

/*
 *
 * @author Brent Easton
 *
 * Enhanced Dice Button includes access to Internet Die Servers via the DieManager.
 */
import org.apache.commons.lang3.ArrayUtils;

import VASSAL.build.AutoConfigurable;
import VASSAL.build.Buildable;
import VASSAL.build.GameModule;
import VASSAL.build.module.documentation.HelpFile;
import VASSAL.command.Command;
import VASSAL.command.CommandEncoder;
import VASSAL.configure.Configurer;
import VASSAL.configure.FormattedStringConfigurer;
import VASSAL.i18n.Resources;

/**
 * This component places a button into the controls window toolbar. Pressing the button generates random numbers and
 * displays the result in the Chatter
 */
public class InternetDiceButton extends DiceButton implements GameComponent, CommandEncoder {
  protected static DieManager dieManager;
  private static final String COMMAND_PREFIX = "SEMAIL\t"; //$NON-NLS-1$
  /** Report format variale */
  public static final String DETAILS = "rollDetails"; //$NON-NLS-1$

  public static String getConfigureTypeName() {
    return Resources::get_string("Editor.InternetDiceButton.component_type"); //$NON-NLS-1$
  }

  @Override
  public Class<?>[] getAttributeTypes() {
    final Class<?>[] c = super.getAttributeTypes();
    for (int i = 0; i < c.length; ++i) {
      if (c[i] == ReportFormatConfig.class) {
        c[i] = InternetReportFormatConfig.class;
      }
    }
    return c;
  }

  public static class InternetReportFormatConfig extends ReportFormatConfig {
    @Override
    public Configurer getConfigurer(AutoConfigurable c, String key, String name) {
      final FormattedStringConfigurer config =
        (FormattedStringConfigurer) super.getConfigurer(c, key, name);
      config.setOptions(ArrayUtils.add(config.getOptions(), DETAILS));
      return config;
    }
  }

  /**
   * Ask the die manager to do our roll!
   */
  @SuppressWarnings("removal")
  @Override
  protected void DR() {
    reportFormat.setProperty(NAME, getLocalizedConfigureName());
    dieManager.roll(nDice, nSides, plus, reportTotal, getLocalizedConfigureName(), reportFormat);
  }

  /**
   * Expects to be added to the DieManager.
   */
  @Override
  public void addTo(Buildable parent) {
    initDieManager();
    dieManager.addDieButton(this);
    GameModule.getGameModule().addCommandEncoder(this);
    GameModule.getGameModule().getGameState().addGameComponent(this);
    super.addTo(parent);
  }

  protected void initDieManager() {
    if (dieManager == null) {
      dieManager = new DieManager();
      dieManager.build(null);
    }
  }

  @Override
  public void removeFrom(Buildable b) {
    dieManager.removeDieButton(this);
    GameModule.getGameModule().removeCommandEncoder(this);
    GameModule.getGameModule().getGameState().removeGameComponent(this);
    super.removeFrom(b);
  }

  @Override
  public void setup(boolean gameStarting) {
  }

  @Override
  public Command getRestoreCommand() {
    return new SetSecondaryEmail(dieManager.getServer().getSecondaryEmail());
  }

  @Override
  public Command decode(String command) {
    if (!command.startsWith(COMMAND_PREFIX)) {
      return null;
    }

    return new SetSecondaryEmail(command.substring(COMMAND_PREFIX.length()));
  }

  @Override
  public String encode(Command c) {
    if (!(c instanceof SetSecondaryEmail)) {
      return null;
    }

    return COMMAND_PREFIX + ((SetSecondaryEmail) c).msg;
  }

  private static class SetSecondaryEmail extends Command {
    private final String msg;

    private SetSecondaryEmail(String s) {
      msg = s;
    }

    @Override
    protected void executeCommand() {
      dieManager.setSecondaryEmail(msg);
    }

    @Override
    protected Command myUndoCommand() {
      return null;
    }
  }

  @Override
  public HelpFile getHelpFile() {
    return HelpFile.getReferenceManualPage("GameModule.html", "InternetDiceButton"); //$NON-NLS-1$ //$NON-NLS-2$
  }
}
