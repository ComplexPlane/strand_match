//@author ComplexPlane
//@description Imports labels found by strand_match.
//@category SMB
//@keybinding
//@menupath
//@toolbar

import ghidra.app.script.GhidraScript;
import ghidra.app.util.exporter.XmlExporter;
import ghidra.program.model.listing.*;
import ghidra.program.model.symbol.*;
import ghidra.program.model.address.*;
import ghidra.util.exception.*;
import java.io.File;
import java.io.BufferedReader;
import java.io.FileReader;

public class StrandMatchImport extends GhidraScript {

	@Override
	protected void run() throws Exception {
		SymbolTable symbolTable = currentProgram.getSymbolTable();
		Namespace globalNs = currentProgram.getGlobalNamespace();

		BufferedReader mapFile = new BufferedReader(new FileReader(askFile("Select a symbol map to import.\nNote that this will overwrite existing symbols at the given addresses.", "Import")));
		String line;

		while ((line = mapFile.readLine()) != null) {
			String[] split = line.split(" ");
			String newSymbolName = split[0];
			String newNamespaceName = split[1];
			Address addr = toAddr(split[2]);

			Namespace ns;
			try {
				ns = symbolTable.createNameSpace(globalNs, newNamespaceName, SourceType.IMPORTED);
			} catch (DuplicateNameException e) {
				ns = symbolTable.getNamespace(newNamespaceName, globalNs);
			}

			Symbol[] userSymbols = symbolTable.getUserSymbols(addr);
			for (Symbol s : userSymbols) {
				symbolTable.removeSymbolSpecial(s);
			}

			symbolTable.createLabel(addr, newSymbolName, ns, SourceType.IMPORTED);
		}

		// removeSymbolSpecial() supposedly will remove symbol references, so re-analyze to restore them
		analyzeAll(currentProgram);
	}
}
