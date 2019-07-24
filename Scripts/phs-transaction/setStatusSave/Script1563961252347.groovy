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

ResponseObject response = WS.sendRequest(findTestObject('phs-transaction/SetStatusSave', [('endpoint') : GlobalVariable.endpoint, ('reqType') : findTestData(
                'phs-transaction/setStatusSave').getValue(1, 1), ('id') : findTestData('phs-transaction/setStatusSave').getValue(
                2, 1), ('status') : findTestData('phs-transaction/setStatusSave').getValue(3, 1), ('trxIn') : findTestData(
                'phs-transaction/setStatusSave').getValue(4, 1), ('userId') : findTestData('phs-transaction/setStatusSave').getValue(
                5, 1), ('trxUpdate') : findTestData('phs-transaction/setStatusSave').getValue(6, 1), ('userUpdate') : findTestData(
                'phs-transaction/setStatusSave').getValue(7, 1)]))

WS.verifyResponseStatusCode(response, 200)