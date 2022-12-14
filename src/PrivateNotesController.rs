/*
 *
 * Copyright (c) 2004 by Rodney Kinney
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
# package VASSAL.build.module.noteswindow;

import java.awt.Component;
import java.util.HashSet;
import java.util.Set;

import javax.swing.Box;
import javax.swing.JLabel;

import VASSAL.build.GameModule;
import VASSAL.build.module.GameComponent;
import VASSAL.command.Command;
import VASSAL.command.CommandEncoder;
import VASSAL.configure.TextConfigurer;
import VASSAL.i18n.Resources;
import VASSAL.tools.SequenceEncoder;

/**
 * Holds {@link PrivateText} objects, only displaying the one owned by the current user
 */
public class PrivateNotesController implements GameComponent, CommandEncoder, SetPrivateTextCommand.Interface {
  public static final String COMMAND_PREFIX = "PNOTE\t"; //$NON-NLS-1$

  private final Set<PrivateText> notes;
  private String myLastSavedNotes;
  private Component controls;
  private TextConfigurer text;

  public PrivateNotesController() {
    notes = new HashSet<>();
  }

  public Component getControls() {
    if (controls == null) {
      final Box b = Box.createVerticalBox();
      b.add(new JLabel(Resources::get_string("Notes.invisible"))); //$NON-NLS-1$
      text = new TextConfigurer(null, null);
      b.add(text.getControls());
      controls = b;
    }
    return controls;
  }

  @Override
  public void addPrivateText(PrivateText p) {
    notes.remove(p);
    notes.add(p);
    if (p.getOwner().equals(GameModule.getActiveUserId())) {
      text.setValue(p.getText());
    }
  }

  @Override
  public Command decode(String command) {
    if (!command.startsWith(COMMAND_PREFIX)) {
      return null;
    }
    final SequenceEncoder.Decoder st = new SequenceEncoder.Decoder(command.substring(COMMAND_PREFIX.length()), '\t');
    final String owner = st.nextToken();
    final String text = st.hasMoreTokens() ? TextConfigurer.restoreNewlines(st.nextToken()) : ""; //$NON-NLS-1$
    return new SetPrivateTextCommand(this, new PrivateText(owner, text));
  }

  @Override
  public String encode(Command c) {
    if (!(c instanceof SetPrivateTextCommand)) {
      return null;
    }
    final PrivateText t = ((SetPrivateTextCommand) c).getPrivateText();
    final SequenceEncoder se = new SequenceEncoder('\t');
    return COMMAND_PREFIX +
      se
        .append(t.getOwner())
        .append(TextConfigurer.escapeNewlines(t.getText()))
        .getValue();
  }

  @Override
  public Command getRestoreCommand() {
    Command comm = null;
    for (final PrivateText privateText : notes) {
      final SetPrivateTextCommand c = new SetPrivateTextCommand(this, privateText);
      if (comm == null) {
        comm = c;
      }
      else {
        comm.append(c);
      }
    }
    return comm;
  }

  @Override
  public void setup(boolean gameStarting) {
    if (!gameStarting) {
      notes.clear();
      text.setValue(""); //$NON-NLS-1$
    }
  }

  public Command save() {
    Command comm = null;
    if (!myLastSavedNotes.equals(text.getValue())) {
      comm = new SetPrivateTextCommand(this, new PrivateText(GameModule.getActiveUserId(), (String) text.getValue()));
      comm.execute();
    }
    return comm;
  }

  public void captureState() {
    myLastSavedNotes = (String) text.getValue();
  }

  public void restoreState() {
    text.setValue((Object) myLastSavedNotes);
  }
}
