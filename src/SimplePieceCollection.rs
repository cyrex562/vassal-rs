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
# package VASSAL.build.module.map;

import java.util.ArrayList;
import java.util.List;

import VASSAL.counters.GamePiece;

/**
 * A SimplePieceCollection is used within a {@link CompoundPieceCollection} to maintain an array of the
 * pieces/stacks/decks ({@link VASSAL.counters.GamePiece}) within a single visual layer. Even within a
 * single "layer", the individual pieces can be rearranged, which will change their relative draw order
 * within that layer.
 */
public class SimplePieceCollection implements PieceCollection {
  private final List<GamePiece> pieces = new ArrayList<>();

  /**
   * Returns the index of a piece.  When painting the map, pieces
   * are drawn in order of index, so lowest index is drawn first and
   * therefore appears at the "bottom", as later pieces are then
   * drawn "on top of" it.
   */
  @Override
  public int indexOf(GamePiece p) {
    return pieces.indexOf(p);
  }

  @Override
  public boolean canMerge(GamePiece p1, GamePiece p2) {
    return true;
  }

  @Override
  public void add(GamePiece p) {
    pieces.add(p);
  }

  @Override
  public void clear() {
    pieces.clear();
  }

  @Override
  public void remove(GamePiece p) {
    removePieceAt(indexOf(p));
  }

  @Override
  public GamePiece[] getPieces() {
    return pieces.toArray(new GamePiece[0]);
  }

  @Override
  public GamePiece[] getAllPieces() {
    return getPieces();
  }

  private void removePieceAt(int gone) {
    if (gone >= 0) {
      pieces.remove(gone);
    }
  }

  public void reposition(GamePiece p, int pos) {
    final int i = pieces.indexOf(p);
    if (i >= 0) {
      pieces.remove(i);
      pieces.add(pos, p);
    }
  }

  @Override
  public void moveToBack(GamePiece p) {
    reposition(p, 0);
  }

  @Override
  public void moveToFront(GamePiece p) {
    final int i = pieces.indexOf(p);
    if (i >= 0) {
      pieces.remove(p);
      pieces.add(p);
    }
  }
}
