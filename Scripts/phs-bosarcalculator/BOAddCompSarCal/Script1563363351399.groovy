import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

ResponseObject response = WS.sendRequest(findTestObject('phs-bosarcalculator/BOAddCompSarCal', [('endpoint') : GlobalVariable.endpoint, ('objid') : findTestData(
                'phs-bosarcalculator/BOAddCompSarCal').getValue(1, 1), ('clntnum') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(
                2, 1), ('currcy') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(3, 1), ('crtable01') : findTestData(
                'phs-bosarcalculator/BOAddCompSarCal').getValue(4, 1), ('rcesagenew') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(
                5, 1), ('rcestrmnew') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(6, 1), ('suminsnew') : findTestData(
                'phs-bosarcalculator/BOAddCompSarCal').getValue(7, 1), ('zsprm') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(
                8, 1), ('chdrnum') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(9, 1), ('rider') : findTestData(
                'phs-bosarcalculator/BOAddCompSarCal').getValue(10, 1), ('crtable02') : findTestData('phs-bosarcalculator/BOAddCompSarCal').getValue(
                11, 1)]))

WS.verifyResponseStatusCode(response, 200)



