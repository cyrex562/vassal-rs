/*
 *
 * Copyright (c) 2000-2009 by Rodney Kinney, Brent Easton
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
# package VASSAL.chat;

import java.util.HashMap;
import java.util.Map;

/**
 * Represents the history and current state of connections to the chat
 * room server
 *
 * @author rkinney
 */
public interface ServerStatus {
  /** Return the current connections to the server */
  ModuleSummary[] getStatus();

  String[] getSupportedTimeRanges();

  /**
   * @return the connections to the server within <code>time</code>
   * milliseconds of the current time
   */
  ModuleSummary[] getHistory(String timeRange);

  class ModuleSummary {
    private String moduleName;
    private final Map<String, Room> rooms = new HashMap<>();

    public ModuleSummary(String moduleName) {
      this.moduleName = moduleName;
    }

    public ModuleSummary(String moduleName, Room[] rooms) {
      this (moduleName);
      for (final Room r : rooms) {
        this.rooms.put(r.get_name(), r);
      }
    }

    public String getModuleName() {
      return moduleName;
    }

    public void setModuleName(String moduleName) {
      this.moduleName = moduleName;
    }

    public void addRoom(Room r) {
      rooms.put(r.get_name(), r);
    }

    public SimpleRoom getRoom(String name) {
      return (SimpleRoom) rooms.get(name);
    }

    public Room[] getRooms() {
      return rooms.values().toArray(new Room[0]);
    }

    public int numPlayers() {
      final Room[] roomsArray = getRooms();
      int n = 0;
      for (final Room room : roomsArray) {
        n += ((SimpleRoom) room).numPlayers();
      }
      return n;
    }

    @Override
    public String toString() {
      return moduleName;
    }
  }
}
