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

ResponseObject response = WS.sendRequest(findTestObject('phs-transaction/saveEvdLifeAssured', [('endpoint') : GlobalVariable.endpoint, ('genId') : findTestData(
                'phs-transaction/saveEvdLifeAssured').getValue(1, 1), ('clientNo') : findTestData('phs-transaction/saveEvdLifeAssured').getValue(
                2, 1), ('role') : findTestData('phs-transaction/saveEvdLifeAssured').getValue(3, 1), ('evdCode') : findTestData(
                'phs-transaction/saveEvdLifeAssured').getValue(4, 1), ('evdDesc') : findTestData('phs-transaction/saveEvdLifeAssured').getValue(
                5, 1), ('trxIn') : findTestData('phs-transaction/saveEvdLifeAssured').getValue(6, 1), ('resource') : findTestData(
                'phs-transaction/saveEvdLifeAssured').getValue(7, 1)]))

WS.verifyResponseStatusCode(response, 200)