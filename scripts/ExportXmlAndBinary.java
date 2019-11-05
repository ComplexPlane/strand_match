//@author ComplexPlane
//@category SMB
//@keybinding
//@menupath
//@toolbar

import ghidra.app.script.GhidraScript;
import ghidra.app.util.exporter.XmlExporter;
import ghidra.program.model.listing.*;
import java.io.File;

public class ExportXmlAndBinary extends GhidraScript {

	@Override
	protected void run() throws Exception {
		Program prog = this.currentProgram;

		// Create a unique name, any will do
		String xmlFilename = prog.getExecutableMD5() + ".xml";
		File xmlFile = new File(xmlFilename);

		XmlExporter exporter = new XmlExporter();
		exporter.export(xmlFile, prog, null, this.monitor);
	}
}
