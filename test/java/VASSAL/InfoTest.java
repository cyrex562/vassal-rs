package VASSAL;

import org.junit.jupiter.api.Test;

import VASSAL.tools.version.VersionUtils;

import static org.junit.jupiter.api.Assertions.*;

public class InfoTest {
  @Test
  public void testIsModuleTooNewYes() {
    assertTrue(Info.isModuleTooNew(VersionUtils.nextMinorVersion(Info.get_version())));
  }

  @Test
  public void testIsModuleTooNewNo() {
    assertFalse(Info.isModuleTooNew(Info.get_version()));
  }

  @Test
  public void testHasOldFormatYes() {
    assertTrue(Info.hasOldFormat("0.0.1"));
  }

  @Test
  public void testHasOldFormatNo() {
    assertFalse(Info.hasOldFormat(Info.get_version()));
  }
}
