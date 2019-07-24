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

ResponseObject response = WS.sendRequest(findTestObject('phs-transaction/saveErrorLog', [('endpoint') : GlobalVariable.endpoint, ('genId') : findTestData(
                'phs-transaction/saveErrorLog').getValue(1, 1), ('message') : findTestData('phs-transaction/saveErrorLog').getValue(
                2, 1), ('taskName') : findTestData('phs-transaction/saveErrorLog').getValue(3, 1), ('trxIn') : findTestData(
                'phs-transaction/saveErrorLog').getValue(4, 1), ('type') : findTestData('phs-transaction/saveErrorLog').getValue(
                5, 1)]))

WS.verifyResponseStatusCode(response, 200)