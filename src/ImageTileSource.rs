/*
 *
 * Copyright (c) 2007-2010 by Joel Uckelman
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

# package VASSAL.tools.image;

import java.awt.Dimension;
import java.awt.image.BufferedImage;

/**
 * An interface for classes which supply image tiles.
 *
 * @author Joel Uckelman
 * @since 3.2.0
 */
public interface ImageTileSource {
  /**
   * Gets an image tile.
   *
   * @param name the image name
   * @param tileX the X coordinate of the tile
   * @param tileY the Y coordinate of the tile
   * @param scale the scale of the tile
   * @return the tile
   *
   * @throws ImageIOException if the tile can't be read
   */
  BufferedImage getTile(
    String name,
    int tileX,
    int tileY,
    double scale) throws ImageIOException;

  /**
   * Gets the size of an image tile.
   *
   * @param name the image name
   * @param tileX the X coordinate of the tile
   * @param tileY the Y coordinate of the tile
   * @param scale the scale of the tile
   * @return the size of the tile
   *
   * @throws ImageIOException if the tile can't be read
   */
  Dimension getTileSize(
    String name,
    int tileX,
    int tileY,
    double scale) throws ImageIOException;

  /**
   * Checks whether an image tile exists.
   *
   * @param name the image name
   * @param tileX the X coordinate of the tile
   * @param tileY the Y coordinate of the tile
   * @param scale the scale of the tile
   *
   * @throws ImageIOException if the image can't be read
   */
  boolean tileExists(
    String name,
    int tileX,
    int tileY,
    double scale) throws ImageIOException;
}
